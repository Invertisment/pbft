use crate::dto::{PrePrepare,Prepare,Commit,ID,State,Shutdown};
use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use std::option::Option;
use std::thread;
use std::thread::JoinHandle;

pub trait TargetNode {
    fn send_pre_prepare(&self, _req: PrePrepare) -> bool;
    fn send_prepare(&self, _req: Prepare) -> bool;
    fn send_commit(&self, _req: Commit) -> bool;
}

#[derive(Debug)]
pub struct Message {
    pub sender_id: ID,
    pub target_id: ID,
    preprepares: Vec<PrePrepare>,
    prepares: Vec<Prepare>,
    commits: Vec<Commit>,
    shutdowns: Vec<Shutdown>,
}

#[derive(Debug)]
pub struct Node {
    id: ID,
    state: State,
    report_sender: Sender<State>,
}

impl Message {
    pub fn preprepare(sender_id: ID, target_id: ID, pp: PrePrepare) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepares: vec![pp],
            prepares: vec![],
            commits: vec![],
            shutdowns: vec![],
        }
    }
    pub fn commit(sender_id: ID, target_id: ID, c: Commit) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepares: vec![],
            prepares: vec![],
            commits: vec![c],
            shutdowns: vec![],
        }
    }
    pub fn shutdown(sender_id: ID, target_id: ID, s: Shutdown) -> Message {
        Message{
            sender_id: sender_id,
            target_id: target_id,
            preprepares: vec![],
            prepares: vec![],
            commits: vec![],
            shutdowns: vec![s],
        }
    }
}

impl Node {
    pub fn spawn(id: ID, report_sender: Sender<State>) -> (JoinHandle<Result<(), String>>, Sender<Message>) {
        let (data_sender, data_receiver) = mpsc::channel();
        let join_handle = thread::spawn(
            move || {
                let node = Node{
                    id: id,
                    report_sender: report_sender.clone(),
                    state: Option::None,
                };
                for msg in data_receiver {
                    println!("[{}] Received {:?}", node.id, msg);
                    let should_shutdown = node.handle(msg);
                    if should_shutdown {
                        print!("Shutdown");
                        break;
                    }
                }
                Ok(())
            });
        (join_handle, data_sender)
    }

    fn handle(&self, message: Message) -> bool {
        for _e in message.shutdowns {
            print!("[{}] Received shutdown request", self.id);
            return true;
        }
        return false;
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node {}!", self.id);
    }
}
