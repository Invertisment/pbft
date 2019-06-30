use crate::dto::{PrePrepare,Prepare,Commit,ID,Tip,Shutdown,NodeRequest};
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use std::option::Option;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc,Mutex,RwLock};
use std::collections::HashSet;
use std::result::{Result};
use crate::util::retain_others;
use crate::reqtable::RequestTable;
use crate::sufficiency::{one,two_thirds};

#[derive(Debug)]
pub struct State {
    tip: Tip, // current consensus viewpoint of the node
    known_nodes: HashSet<ID>,
    preprepares: RequestTable<PrePrepare>,
    prepares: RequestTable<Prepare>,
    commits: RequestTable<Commit>,
}

impl State {
    pub fn genesis(known_nodes: HashSet<ID>) -> Arc<Mutex<State>> {
        Arc::new(Mutex::new(State{
            tip: Option::None,
            preprepares: RequestTable::new(one),
            prepares: RequestTable::new(two_thirds),
            commits: RequestTable::new(two_thirds),
            known_nodes: known_nodes,
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

    fn handle_message<M>(reqtable: &mut RequestTable<M>, maybe_msg: &Option<Arc<RwLock<M>>>) -> Result<(), String>
    where M: NodeRequest {
        match maybe_msg {
            Some(msg) => {
                reqtable.append(msg.clone())
            },
            None => Ok(())
        }
    }

    pub fn handle_protocol_message(&mut self, message: &Message) -> Result<(), String> {
        print!("new message! {:?}", &message);
        // TODO: Not sure how to make a for loop here
        if message.preprepare.is_some() {
            return State::handle_message(&mut self.preprepares, &message.preprepare);
        }
        if message.prepare.is_some() {
            return State::handle_message(&mut self.prepares, &message.prepare);
        }
        if message.commit.is_some() {
            return State::handle_message(&mut self.commits, &message.commit);
        }
        Ok(())
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
        let state = State::genesis(retain_others(id, known_nodes));
        let state_clone = state.clone();
        let join_handle = thread::spawn(
            move || {
                let node = Node {
                    id: id,
                    state: state.clone(),
                };
                node.handle_all_requests(data_receiver)
                });
        NodeCtrl {
            join_handle: join_handle,
            data_sender: data_sender,
            state: state_clone
        }
    }

    fn handle_all_requests(&self, data_receiver: Receiver<Message>) -> Result<(), String> {
        for msg in data_receiver {
            //println!("[{}] Received {:?}", node.id, msg);
            let should_shutdown = self.handle_control_message(&msg);
            if should_shutdown {
                print!("[{}] Shutdown", self.id);
                break;
            }
            self.handle_protocol_message(&msg);
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

    fn handle_protocol_message(&self, message: &Message) {
        match self.state.lock() {
            Ok(mut guard) => {
                (*guard).handle_protocol_message(message);
            },
            Err(e) => {
                println!("[{}] Error while trying to acquire state: {:?}", self.id, e);
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
