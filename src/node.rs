use crate::dto::{PrePrepare,Prepare,Commit,ID,Tip,Shutdown};
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use std::option::Option;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc,Mutex};
use std::collections::HashMap;

pub trait TargetNode {
    fn send_pre_prepare(&self, _req: PrePrepare) -> bool;
    fn send_prepare(&self, _req: Prepare) -> bool;
    fn send_commit(&self, _req: Commit) -> bool;
}

#[derive(Debug)]
pub struct Message {
    pub sender_id: ID,
    pub target_id: ID,
    preprepare: Option<PrePrepare>,
    prepare: Option<Prepare>,
    commit: Option<Commit>,
    shutdown: Option<Shutdown>,  // control packet
}

#[derive(Debug)]
pub struct State {
    tip: Tip, // current consensus viewpoint of the node
    preprepares: HashMap<ID, PrePrepare>,
    prepares: HashMap<ID, Prepare>,
    commits: HashMap<ID, Commit>,
}

#[derive(Debug)]
pub struct Node {
    id: ID,
    state: Arc<Mutex<State>>,
}

impl Message {
    pub fn preprepare(sender_id: ID, target_id: ID, pp: PrePrepare) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepare: Option::Some(pp),
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
            prepare: Option::Some(p),
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
            commit: Option::Some(c),
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
            shutdown: Option::Some(s),
        }
    }
}

#[derive(Debug)]
pub struct NodeCtrl {
    pub join_handle: JoinHandle<Result<(), String>>,
    pub data_sender: Sender<Message>,
    pub state: Arc<Mutex<State>>,
}

impl Node {
    pub fn spawn(id: ID) -> NodeCtrl {
        let (data_sender, data_receiver) = mpsc::channel();
        let state = State::genesis();
        let state_clone = state.clone();
        let join_handle = thread::spawn(
            move || {
                let node = Node{
                    id: id,
                    state: state.clone(),
                };
                node.handle_all_requests(data_receiver)
            });
        NodeCtrl{
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
        message.shutdown.is_some()
    }

    fn handle_protocol_message(&self, _message: &Message) {
        // TODO
    }

}

impl Drop for Node {
    fn drop(&mut self) {
        //println!("Dropping Node {}!", self.id);
    }
}

impl State {
    pub fn genesis() -> Arc<Mutex<State>> {
        Arc::new(Mutex::new(State{
            tip: Option::None,
            preprepares: HashMap::new(),
            prepares: HashMap::new(),
            commits: HashMap::new(),
        }))
    }
}
