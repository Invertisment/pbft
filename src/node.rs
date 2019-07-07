use crate::dto::{PrePrepare,Prepare,Commit,ID,Tip,Shutdown,NodeRequest};
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use std::option::Option;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc,Mutex,RwLock};
use std::collections::HashSet;
use std::result::{Result};
use crate::util::find_others;
use crate::reqtable::RequestTable;
use crate::sufficiency::{one,two_thirds};
use crate::util::convert_err;

#[derive(Debug)]
pub struct State {
    tip: Tip, // current consensus viewpoint of the node
    seq_id: ID,
    known_nodes: HashSet<ID>,
    preprepares: RequestTable<PrePrepare>,
    prepares: RequestTable<Prepare>,
    commits: RequestTable<Commit>,
    sent_preprepare: Option<Arc<RwLock<PrePrepare>>>,
    sent_prepare: Option<Arc<RwLock<Prepare>>>,
    sent_commit: Option<Arc<RwLock<Commit>>>,
}

impl State {
    pub fn genesis(known_nodes: HashSet<ID>) -> Arc<Mutex<State>> {
        Arc::new(Mutex::new(State{
            tip: Option::None,
            seq_id: 0,
            preprepares: RequestTable::new(one),
            prepares: RequestTable::new(two_thirds),
            commits: RequestTable::new(two_thirds),
            known_nodes: known_nodes,
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

    fn is_valid_next_seq(&self, req: &NodeRequest) -> bool {
        req.get_seq_id() > self.seq_id
    }

    fn handle_preprepare(&mut self, me: ID, message: Arc<RwLock<PrePrepare>>, _data_sender: Sender<Message>) -> Result<(), String> {
        // Did we sent any preprepares for this round? We should send only one next preprepare
        if self.sent_preprepare.is_some() {
            return Ok(())
        }
        let result = self.preprepares.append(message.clone());
        if result.is_err() {
            return result;
        }
        convert_err(message.read()).map(|message_lock| {
            // Was the inserted message valid?
            println!("[{:?}] Preprepare sufficiency {:?}", me, self.preprepares.is_sufficient(&message_lock, &self.known_nodes));
            if !self.preprepares.is_sufficient(&message_lock, &self.known_nodes) {
                return
            }
            // seq_id must point to the future one
            if !self.is_valid_next_seq(&*message_lock) {
                println!("[{:?}] Preprepare dropped", me);
                return;
            }
            // sufficient and fresh preprepare
            println!("[{:?}] Preprepare is sufficient! Sending to {:?}", me, self.known_nodes);
        })
    }

    fn handle_prepare(&mut self, _me: ID, message: Arc<RwLock<Prepare>>, _data_sender: Sender<Message>) -> Result<(), String> {
        let result = self.prepares.append(message.clone());
        result.map(|_a| ())
        //self.prepares.is_sufficient(message, &self.known_nodes)
        //    .map(|is_sufficient| {
        //        println!("[{:?}] Prepare is sufficient! Sending to {:?}", me, self.known_nodes);
        //    })
    }

    fn handle_commit(&mut self, _me: ID, message: Arc<RwLock<Commit>>, _data_sender: Sender<Message>) -> Result<(), String> {
        let result = self.commits.append(message.clone());
        result.map(|_a| ())
        //self.commits.is_sufficient(message, &self.known_nodes)
        //    .map(|is_sufficient| {
        //        println!("[{:?}] Commit is sufficient! Sending to {:?}", me, self.known_nodes);
        //    })
    }

    pub fn handle_protocol_message(&mut self, me: ID, message: Message, data_sender: Sender<Message>) -> Result<(), String> {
        print!("new message! {:?}", &message);
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

fn wrap_to_arc_option<T>(t: T) -> Option<Arc<RwLock<T>>> {
    Option::Some(Arc::new(RwLock::new(t)))
}

#[derive(Debug)]
pub struct Message {
    sender_id: ID,
    target_id: ID,
    preprepare: Option<Arc<RwLock<PrePrepare>>>,
    prepare: Option<Arc<RwLock<Prepare>>>,
    commit: Option<Arc<RwLock<Commit>>>,
    shutdown: Option<Arc<RwLock<Shutdown>>>,  // control packet
}

impl Message {
    pub fn preprepare(sender_id: ID, target_id: ID, pp: PrePrepare) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepare: wrap_to_arc_option(pp),
            prepare: Option::None,
            commit: Option::None,
            shutdown: Option::None,
        }
    }
    pub fn prepare(sender_id: ID, target_id: ID, p: Prepare) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepare: Option::None,
            prepare: wrap_to_arc_option(p),
            commit: Option::None,
            shutdown: Option::None,
        }
    }
    pub fn commit(sender_id: ID, target_id: ID, c: Commit) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepare: Option::None,
            prepare: Option::None,
            commit: wrap_to_arc_option(c),
            shutdown: Option::None,
        }
    }
    pub fn shutdown(sender_id: ID, target_id: ID, s: Shutdown) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepare: Option::None,
            prepare: Option::None,
            commit: Option::None,
            shutdown: wrap_to_arc_option(s),
        }
    }

    pub fn get_target_id(&self) -> ID {
        self.target_id
    }
}

#[derive(Debug)]
pub struct Node {
    id: ID,
    state: Arc<Mutex<State>>,
}

impl Node {
    pub fn spawn(id: ID, known_nodes: &HashSet<ID>) -> NodeCtrl {
        let (data_sender, data_receiver) = mpsc::channel();
        let state = State::genesis(find_others(id, known_nodes.iter()).collect());
        let state_clone = state.clone();
        let data_sender_clone = data_sender.clone();
        let join_handle = thread::spawn(
            move || {
                let node = Node {
                    id: id,
                    state: state.clone(),
                };
                node.handle_all_requests(data_receiver, data_sender_clone)
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
