use crate::dto::{Commit,ID};
use std::collections::HashSet;

pub fn new_req(view: ID, seq: ID, sender: ID) -> Commit {
    Commit::new(
        view as ID,
        seq as ID,
        format!("Digest {} {}", view, seq).to_owned(),
        sender as ID,
        sender as ID)
}

pub fn new_nodes(count: usize) -> HashSet<ID> {
    let mut nodes: HashSet<ID> = HashSet::new();
    for i in 0..count as ID {
        nodes.insert(i);
    }
    return nodes;
}
