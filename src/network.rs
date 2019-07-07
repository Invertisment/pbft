use crate::node::{Node,Message,NodeCtrl,State};
use crate::dto::{ID,Shutdown};
use std::collections::{HashMap,HashSet,VecDeque};
use std::thread::JoinHandle;
use std::sync::{Arc,RwLock,Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver,TryRecvError};
use std::iter::{Iterator};

#[derive(Debug)]
pub struct Network {
    nodes: HashMap<ID, NodeCtrl>,
    inter_receiver: Receiver<Message>,
    queue: VecDeque<Message>,
}

fn create_nodes(size: usize) -> (HashMap<ID, NodeCtrl>, Receiver<Message>) {
    let (inter_sender, inter_receiver) = mpsc::channel();
    let node_ids: &HashSet<ID> = &(0..size as ID).into_iter().collect();
    let mut nodes: HashMap<ID, NodeCtrl> = HashMap::new();
    for i in node_ids {
        nodes.insert(*i, Node::spawn(*i, node_ids, inter_sender.clone()));
    }
    return (nodes, inter_receiver);
}

impl Network {
    pub fn new(size: usize) -> Network {
        let (nodes, inter_receiver) = create_nodes(size);
        Network{
            nodes: nodes,
            inter_receiver: inter_receiver,
            queue: VecDeque::new(),
        }
    }

    pub fn tick(&mut self) -> Result<bool, String> {
        if self.nodes.len() == 0 {
            return Err("No nodes were found".to_owned());
        }
        match self.queue.pop_front() {
            Some(req) => {
                //println!("[Network] Processing request");
                return self.send(req);
            },
            None => {
                return Err("No more requests".to_owned());
            }
        }
    }

    pub fn tick_queue_all(&mut self) {
        while !self.queue.is_empty() {
            let _ = self.tick();
        }
    }

    pub fn queue_update(&mut self) {
        for _ in 0..10 {
            match self.inter_receiver.try_recv() {
                Ok(message) => self.queue.push_back(message),
                Err(err_type) => {
                    match err_type {
                        TryRecvError::Disconnected => println!("[Network] Receiver shut down"),
                        TryRecvError::Empty => {}
                    }
                    break;
                }
            }
        }
    }

    pub fn tick_until_empty_skip_queue(&mut self) {
        loop {
            match self.inter_receiver.try_recv() {
                Ok(message) => {
                    let _= self.send(message);
                },
                Err(err_type) => {
                    match err_type {
                        TryRecvError::Disconnected => println!("[Network] Receiver shut down"),
                        TryRecvError::Empty => {}
                    }
                    break;
                }
            }
        }
    }

    pub fn queue_add(&mut self, req: Message) {
        self.queue.push_back(req)
    }

    fn send(&mut self, req: Message) -> Result<bool, String> {
        self.send_to_node(req.get_target_id(), req)
    }

    fn send_to_node(&mut self, id: ID, req: Message) -> Result<bool, String> {
        let maybe_node_data: Option<&NodeCtrl> = self.nodes.get(&id);
        match maybe_node_data {
            Some(node_data) => match node_data.get_data_sender().send(req) {
                Ok(()) => Ok(true),
                Err(e) => Err(format!("Can't send: {:?}", e)),
            },
            None => Ok(false),
        }
    }

    pub fn remove_node(&mut self, id: ID) -> Option<JoinHandle<Result<(), String>>> {
        let node_res = self.send_to_node(id, Message::shutdown(0 as ID, 0 as ID, Arc::new(RwLock::new(Shutdown{}))));
        let tuple: Option<NodeCtrl> = match node_res {
            Ok(_b) => self.nodes.remove(&id),
            Err(_e) => None,
        };
        tuple.map(|t| t.get_join_handle())
    }

    pub fn get_statuses<'a>(&'a self) -> impl Iterator<Item = (&ID, Arc<Mutex<State>>)> + 'a {
        self.nodes.iter().map(|(id, node_ctrl)| {
            (id, node_ctrl.get_state())
        })
    }

    pub fn get_node(&self, id: &ID) -> Option<&NodeCtrl> {
        self.nodes.get(id)
    }

    pub fn get_queue<'a>(&'a self) -> impl Iterator<Item = &Message> + 'a {
        self.queue.iter()
    }

    pub fn get_nodes(&self) -> HashSet<ID> {
        self.nodes.keys().map(|k| *k).collect()
    }
}
