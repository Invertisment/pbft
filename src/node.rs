use crate::dto::{PrePrepare,Prepare,Commit,NodeID,ID,Tip,Shutdown,NodeRequest};
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use std::option::Option;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc,Mutex,RwLock,RwLockReadGuard};
use std::collections::HashSet;
use std::result::{Result};
use crate::util::find_others;
use crate::reqtable::RequestTable;
use crate::sufficiency::{one,two_thirds};
use crate::util::{convert_err,digest};

#[derive(Debug)]
pub struct State {
    tip: Tip, // current consensus viewpoint of the node
    seq_id: ID,
    remaining_nodes: HashSet<ID>,
    all_nodes: HashSet<ID>,
    preprepares: RequestTable<PrePrepare>,
    prepares: RequestTable<Prepare>,
    commits: RequestTable<Commit>,
    sent_preprepare: Option<Arc<RwLock<PrePrepare>>>,
    sent_prepare: Option<Arc<RwLock<Prepare>>>,
    sent_commit: Option<Arc<RwLock<Commit>>>,
}

impl State {
    pub fn genesis(me: ID, all_nodes: HashSet<ID>) -> Arc<Mutex<State>> {
        let remaining_nodes = find_others(me, all_nodes.iter()).collect();
        Arc::new(Mutex::new(State{
            tip: "genesis".to_owned(),
            seq_id: 0,
            preprepares: RequestTable::new(one),
            prepares: RequestTable::new(two_thirds),
            commits: RequestTable::new(two_thirds),
            remaining_nodes: remaining_nodes,
            all_nodes: all_nodes,
            sent_preprepare: None,
            sent_prepare: None,
            sent_commit: None
        }))
    }

    pub fn get_preprepares(&self) -> &RequestTable<PrePrepare> {
        &self.preprepares
    }

    pub fn get_prepares(&self) -> &RequestTable<Prepare> {
        &self.prepares
    }

    pub fn get_commits(&self) -> &RequestTable<Commit> {
        &self.commits
    }

    fn send<M>(&self, me: ID, data_sender: Sender<Message>, conversion_fn: fn(ID, ID, Arc<RwLock<M>>) -> Message, request: Arc<RwLock<M>>) {
        // Error handling: fire and forget - UDP mode
        for m in Message::multiply(conversion_fn, request, me, &self.remaining_nodes) {
            //println!("[{:?}] Sending to network", me);
            let res = data_sender.send(m);
            if res.is_err() {
                println!("[{:?}] Send error: {:?}", me, res.err())
            }
        }
    }

    fn is_valid_next_seq(&self, req: &NodeRequest) -> bool {
        req.get_seq_id() > self.seq_id
    }

    fn append<M>(reqs: &mut RequestTable<M>, sent_request: &Option<Arc<RwLock<M>>>, message: &Arc<RwLock<M>>) -> Result<(), String>  where M: NodeRequest {
        // Did we sent any preprepares for this round? We should send only one next preprepare
        if sent_request.is_some() {
            return Ok(())
        }
        reqs.append(message.clone())
    }

    fn validate_message<M, N>(&self,me: ID, reqs: &RequestTable<M>,  message: &N) -> bool
    where M: NodeRequest + std::fmt::Debug,
          N: NodeRequest + std::fmt::Debug
    {
        println!("[{:?}: {} -> {}; [{:?}]] Msg: {:?}", me, me, message.get_sender_id(), self.all_nodes, message);
        // Was the inserted message valid?
        //println!("[{:?}] Preprepare sufficiency {:?}", me, self.preprepares.is_sufficient(&message_lock, &self.all_nodes));
        if !reqs.is_sufficient(message, &self.all_nodes) {
            println!("[{:?}] Drop: message doesn't have enough approvers", me);
            return false
        }
        // seq_id must point to the future one
        if !self.is_valid_next_seq(message) {
            println!("[{:?}] Drop: next seq is invalid", me);
            return false;
        }
        true
    }

    fn handle_preprepare(&mut self, me: ID, message: Arc<RwLock<PrePrepare>>, data_sender: Sender<Message>) -> Result<(), String> {
        let result = State::append(&mut self.preprepares, &self.sent_preprepare, &message);
        if result.is_err() {
            return result;
        }
        convert_err(message.read()).map(|_message_lock| {
            let message_lock: RwLockReadGuard<PrePrepare> = _message_lock;
            if !self.validate_message(me, &self.preprepares, &*message_lock) {
                return;
            }
            // if we've sent anything we shouldn't send anything twice
            if self.sent_prepare.is_some() {
                println!("[{:?}] Preprepare drop: next seq is invalid", me);
                return;
            }
            // new prepare
            let prepare = Arc::new(RwLock::new(message_lock.make_prepare(me)));
            // handle our new prepare internally
            let res = self.handle_prepare(me, prepare.clone(), data_sender.clone());
            if res.is_err() {
                println!("[{:?}] Prepare insertion err {:?}", me, res.err());
                return;
            }
            //println!("[{:?}] Preprepare is sufficient! Sending to {:?}", me, self.all_nodes);
            self.send(me, data_sender, Message::prepare, prepare);
        })
    }

    fn handle_prepare(&mut self, me: ID, message: Arc<RwLock<Prepare>>, data_sender: Sender<Message>) -> Result<(), String> {
        let result = State::append(&mut self.prepares, &self.sent_prepare, &message);
        if result.is_err() {
            return result;
        }
        convert_err(message.read()).map(|_message_lock| {
            let message_lock: RwLockReadGuard<Prepare> = _message_lock;
            if !self.validate_message(me, &self.preprepares, &*message_lock) {
                return;
            }
            // if we've sent anything we shouldn't send anything twice
            if self.sent_commit.is_some() {
                println!("[{:?}] Prepare drop: next seq is invalid", me);
                return;
            }
            // new prepare
            let commit = Arc::new(RwLock::new(message_lock.make_commit(me, digest(me))));
            // handle our new prepare internally
            let res = self.handle_commit(me, commit.clone(), data_sender.clone());
            if res.is_err() {
                println!("[{:?}] Prepare insertion err {:?}", me, res.err());
                return;
            }
            //println!("[{:?}] Prepare is sufficient! Sending to {:?}", me, self.all_nodes);
            self.send(me, data_sender, Message::commit, commit);
        })
    }

    fn update_tip(&mut self, me: ID, commit: &Commit) {
        // check that all preprepares and prepares exist
        if !self.preprepares.is_sufficient(commit, &self.all_nodes)
            && !self.prepares.is_sufficient(commit, &self.all_nodes) {
                println!("[{:?}] Commit ignore: previous requests are not sufficient", me);
                return
            }
        let found_p = self.preprepares.find(commit);
        let new_state: Option<String> = found_p
            .map(|preprepare_lock|
                 preprepare_lock
                 .read()
                 .map(|preprepare| preprepare.get_message())
                 .ok())
            .unwrap();
        // save the new state
        self.tip = new_state.unwrap_or(self.tip.clone());
    }

    fn handle_commit(&mut self, me: ID, message: Arc<RwLock<Commit>>, _data_sender: Sender<Message>) -> Result<(), String> {
        let result = State::append(&mut self.commits, &self.sent_commit, &message);
        if result.is_err() {
            return result;
        }
        convert_err(message.read()).map(|_message_lock| {
            let message_lock: RwLockReadGuard<Commit> = _message_lock;
            if !self.validate_message(me, &self.prepares, &*message_lock) {
                return;
            }
            // if we've sent anything we shouldn't send anything twice
            if self.sent_commit.is_some() {
                println!("[{:?}] Prepare drop: next seq is invalid", me);
                return;
            }
            println!("[{:?}] Client response: {:?}", me, message_lock);
            self.update_tip(me, &*message_lock)
        })
    }

    pub fn handle_protocol_message(&mut self, me: ID, message: Message, data_sender: Sender<Message>) -> Result<(), String> {
        //print!("new message! {:?}", &message);
        // TODO: Not sure how to make a for loop here; don't want to create new structs
        if message.preprepare.is_some() {
            return self.handle_preprepare(me, message.preprepare.unwrap(), data_sender)
        }
        if message.prepare.is_some() {
            return self.handle_prepare(me, message.prepare.unwrap(), data_sender)
        }
        if message.commit.is_some() {
            return self.handle_commit(me, message.commit.unwrap(), data_sender)
        }
        Err("Unknown message".to_owned())
    }
}

#[derive(Debug)]
pub struct Message {
    sender_id: NodeID,
    target_id: NodeID,
    preprepare: Option<Arc<RwLock<PrePrepare>>>,
    prepare: Option<Arc<RwLock<Prepare>>>,
    commit: Option<Arc<RwLock<Commit>>>,
    shutdown: Option<Arc<RwLock<Shutdown>>>,  // control packet
}

impl Message {
    pub fn multiply<'a, M>(conversion_fn: fn(NodeID, NodeID, Arc<RwLock<M>>) -> Message, req: Arc<RwLock<M>>, sender: NodeID, nodes: &'a HashSet<ID>) -> impl Iterator<Item = Message> + 'a where M: 'a {
        nodes.iter().map(move |target_node_id| {
            conversion_fn(sender, *target_node_id, req.clone())
        })
    }
    pub fn preprepare(sender_id: NodeID, target_id: ID, pp: Arc<RwLock<PrePrepare>>) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepare: Option::from(pp),
            prepare: Option::None,
            commit: Option::None,
            shutdown: Option::None,
        }
    }
    pub fn prepare(sender_id: NodeID, target_id: ID, p: Arc<RwLock<Prepare>>) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepare: Option::None,
            prepare: Option::from(p),
            commit: Option::None,
            shutdown: Option::None,
        }
    }
    pub fn commit(sender_id: NodeID, target_id: ID, c: Arc<RwLock<Commit>>) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepare: Option::None,
            prepare: Option::None,
            commit: Option::from(c),
            shutdown: Option::None,
        }
    }
    pub fn shutdown(sender_id: NodeID, target_id: ID, s: Arc<RwLock<Shutdown>>) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepare: Option::None,
            prepare: Option::None,
            commit: Option::None,
            shutdown: Option::from(s),
        }
    }

    pub fn get_target_id(&self) -> NodeID {
        self.target_id
    }
}

#[derive(Debug)]
pub struct Node {
    id: ID,
    state: Arc<Mutex<State>>,
}

impl Node {
    pub fn spawn(id: ID, all_nodes: &HashSet<ID>, inter_sender: Sender<Message>) -> NodeCtrl {
        let (data_sender, data_receiver) = mpsc::channel();
        let state = State::genesis(id, all_nodes.iter().map(|i| *i).collect());
        let state_clone = state.clone();
        let join_handle = thread::spawn(
            move || {
                let node = Node {
                    id: id,
                    state: state.clone(),
                };
                node.handle_all_requests(data_receiver, inter_sender)
                });
        NodeCtrl {
            join_handle: join_handle,
            data_sender: data_sender,
            state: state_clone
        }
    }

    fn handle_all_requests(&self, data_receiver: Receiver<Message>, data_sender: Sender<Message>) -> Result<(), String> {
        for msg in data_receiver {
            //println!("[{}] Received {:?}", node.id, msg);
            let should_shutdown = self.handle_control_message(&msg);
            if should_shutdown {
                print!("[{}] Shutdown", self.id);
                break;
            }
            self.handle_protocol_message(msg, data_sender.clone());
        }
        Ok(())
    }

    fn handle_control_message(&self, message: &Message) -> bool {
        //print!("[{}] Received shutdown request", self.id);
        if message.shutdown.is_some() {
            return true
        }
        false
    }

    fn handle_protocol_message(&self, message: Message, data_sender: Sender<Message>) {
        match self.state.lock() {
            Ok(mut guard) => {
                match (*guard).handle_protocol_message(self.id, message, data_sender) {
                    Ok(_ok) => (),
                    Err(e) => println!("[{}] Error in message loop: {:?}", self.id, e),
                }
            },
            Err(e) => {
                println!("[{}] Error while trying to acquire node's own state: {:?}", self.id, e);
            },
        }
    }
}

#[derive(Debug)]
pub struct NodeCtrl {
    join_handle: JoinHandle<Result<(), String>>,
    data_sender: Sender<Message>,
    state: Arc<Mutex<State>>,
}

impl NodeCtrl {
    pub fn get_join_handle(self) -> JoinHandle<Result<(), String>> {
        self.join_handle
    }
    pub fn get_data_sender(&self) -> Sender<Message>{
        self.data_sender.clone()
    }
    pub fn get_state(&self) -> Arc<Mutex<State>>{
        self.state.clone()
    }
}
