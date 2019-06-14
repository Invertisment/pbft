use crate::node::{Node,Message};
use crate::dto::{ID,State,Shutdown};
use std::collections::{HashMap,VecDeque};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver,Sender};
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Network {
    nodes: HashMap<ID, (JoinHandle<Result<(), String>>, Sender<Message>)>,
    statuses: HashMap<ID, String>,
    queue: VecDeque<Message>,
}

// TODO: This message lifetime probably will cause a memory leak
fn create_nodes<'l>(size: usize) -> (HashMap<ID, (JoinHandle<Result<(), String>>, Sender<Message>)>, Receiver<State>) {
    let (report_sender, report_rcv) = mpsc::channel();
    let mut nodes: HashMap<ID, (JoinHandle<Result<(), String>>, Sender<Message>)> = HashMap::new();
    for i in 0..size {
        nodes.insert(i as ID, Node::spawn(i as ID, report_sender.clone()));
    }
    return (nodes, report_rcv);
}

impl Network {
    pub fn new(size: usize, buffer_size: usize) -> Network {
        let (nodes, _report_rcv) = create_nodes(size);
        Network{
            nodes: nodes,
            statuses: HashMap::new(),
            queue: VecDeque::with_capacity(buffer_size),
        }
    }

    pub fn tick(&mut self) -> Result<bool, String> {
        if self.nodes.len() == 0 {
            return Err("No nodes were found".to_owned());
        }
        match self.queue.pop_front() {
            Some(req) => {
                return self.send(req);
            },
            None => {
                return Err("No more requests".to_owned());
            }
        }
    }

    pub fn queue_add(&mut self, req: Message) {
        self.queue.push_back(req)
    }

    fn send(&mut self, req: Message) -> Result<bool, String> {
        self.send_to_node(req.target_id, req)
    }

    fn send_to_node(&mut self, id: ID, req: Message) -> Result<bool, String> {
        let maybe_node_data: Option<&(JoinHandle<Result<(), String>>, Sender<Message>)> = self.nodes.get(&id);
        match maybe_node_data {
            Some(node_data) => match node_data.1.send(req) {
                Ok(()) => Ok(true),
                Err(e) => Err(format!("Can't send: {:?}", e)),
            },
            None => Ok(false),
        }
    }

    pub fn remove_node(&mut self, id: ID) -> Option<JoinHandle<Result<(), String>>> {
        let node_res = self.send_to_node(id, Message::shutdown(0 as ID, 0 as ID, Shutdown{}));
        let tuple: Option<(JoinHandle<Result<(), String>>, Sender<Message>)> = match node_res {
            Ok(_b) => self.nodes.remove(&id),
            Err(_e) => None,
        };
        tuple.map(|t| t.0)
    }
}
