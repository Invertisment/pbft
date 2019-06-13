use crate::node::{Node};
use crate::dto::{Num};
use std::boxed::Box;
use std::collections::{HashMap,VecDeque};

pub trait Request : std::fmt::Debug {
    fn execute(&self, nodes: &HashMap<Num,Node>) -> bool;
}

#[derive(Debug)]
pub struct Network {
    pub nodes: HashMap<Num,Node>,
    pub requests: VecDeque<Box<dyn Request>>,
}

impl Network {
    pub fn new(nodes: HashMap<Num,Node>, requests: VecDeque<Box<dyn Request>>) -> Network {
        Network{nodes, requests}
    }
    pub fn tick(&mut self) -> bool {
        match &mut self.requests.pop_front() {
            Some(req) => {
                return req.execute(&self.nodes); // HTTP status..? no, we're not
            },
            None => {
                return false;
            }
        }
    }
}
