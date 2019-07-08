use crate::dto::{PrePrepare,Commit,ID};
use std::collections::HashSet;
use std::time::Instant;

fn random() -> ID {
    Instant::now().elapsed().as_secs() as ID
}

pub fn new_random_preprepare() -> PrePrepare {
    let sender_id = random();
    PrePrepare::new(
        random(),    // v
        random(),     // n
        "Tip message".to_owned(),    // m
        sender_id,
    )
}

pub fn new_req(view: ID, seq: ID, sender: ID) -> Commit {
    Commit::new(
        view as ID,
        seq as ID,
        sender as ID)
}

pub fn new_nodes(count: usize) -> HashSet<ID> {
    let mut nodes: HashSet<ID> = HashSet::new();
    for i in 0..count as ID {
        nodes.insert(i);
    }
    return nodes;
}
