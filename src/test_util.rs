use crate::dto::{Commit,ID};
use std::collections::HashSet;
use std::sync::{Arc,RwLock};

pub fn new_req(view: ID, seq: ID, sender: ID) -> Arc<RwLock<Commit>> {
    let pre = Commit::new(
        view as ID,
        seq as ID,
        format!("Digest {} {}", view, seq).to_owned(),
        sender as ID,
        sender as ID);
    Arc::new(RwLock::new(pre))
}

pub fn new_nodes(count: usize) -> HashSet<ID> {
    let mut nodes: HashSet<ID> = HashSet::new();
    for i in 0..count as ID {
        nodes.insert(i);
    }
    return nodes;
}

pub fn new_nodes_vec<'a>(count: usize) -> Vec<ID> {
    let mut nodes: Vec<ID> = Vec::new();
    for i in 0..count as ID {
        nodes.push(i);
    }
    return nodes;
}
