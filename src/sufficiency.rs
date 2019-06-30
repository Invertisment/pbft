use crate::dto::{ID};
use std::collections::{HashSet};
use std::iter::Iterator;

// |R| = 3f + 1
// accepted = 2f + 1
// f = (|R| - 1) / 3
// f_61 = (61 - 1) / 3 = 20
// accepted_61 = 2 * f_61 + 1 = 2 * 20 + 1 = 41
// At least 41 nodes of 61 have to accept to have a valid outcome
pub fn two_thirds(all_nodes: &HashSet<ID>, approver_nodes: &Vec<ID>) -> bool {
    clean_noise_approvers(all_nodes, approver_nodes).count() > (((all_nodes.len()) * 2) / 3)
}

pub fn one(all_nodes: &HashSet<ID>, approver_nodes: &Vec<ID>) -> bool {
    let clean: Vec<&ID> = clean_noise_approvers(all_nodes, approver_nodes)
        .take(1)
        .collect();
    clean.first().is_some()
}

fn clean_noise_approvers<'a>(all_nodes: &'a HashSet<ID>, approver_nodes: &'a Vec<ID>) -> impl Iterator<Item = &'a ID> + 'a {
    approver_nodes
        .iter()
        .filter(move |approver_id| all_nodes.contains(approver_id))
}

#[cfg(test)]
use crate::test_util::{new_nodes};

#[test]
fn should_clean_noise_approvers() {
    let nodes = new_nodes(20);
    let approvers = vec![101, 102, 103, 105, 19, 1337, 1338, 20];
    assert_eq!(clean_noise_approvers(&nodes, &approvers).collect::<Vec<&ID>>(), vec![&19]);
}

