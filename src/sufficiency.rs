use crate::dto::{ID};
use std::collections::{HashSet};
use std::iter::Iterator;

pub type SufficiencyChecker = fn(&HashSet<ID>, &HashSet<ID>) -> bool;

// |R| = 3f + 1
// accepted = 2f + 1
// f = (|R| - 1) / 3
// f_61 = (61 - 1) / 3 = 20
// accepted_61 = 2 * f_61 + 1 = 2 * 20 + 1 = 41
// At least 41 nodes of 61 have to accept to have a valid outcome
pub fn two_thirds(all_nodes: &HashSet<ID>, approver_nodes: &HashSet<ID>) -> bool {
    clean_noise_approvers(all_nodes, approver_nodes).count() > (((all_nodes.len()) * 2) / 3)
}

pub fn one(all_nodes: &HashSet<ID>, approver_nodes: &HashSet<ID>) -> bool {
    let clean: Vec<&ID> = clean_noise_approvers(all_nodes, approver_nodes)
        .take(1)
        .collect();
    clean.first().is_some()
}

fn clean_noise_approvers<'a>(all_nodes: &'a HashSet<ID>, approver_nodes: &'a HashSet<ID>) -> impl Iterator<Item = &'a ID> + 'a {
    all_nodes.intersection(approver_nodes)
}

#[cfg(test)]
use crate::test_util::{new_nodes};

#[test]
fn should_clean_noise_approvers() {
    let nodes = new_nodes(20);
    let approvers: HashSet<ID> = [101, 102, 103, 105, 19, 1337, 1338, 20].iter().map(|i| *i).collect();
    assert_eq!(clean_noise_approvers(&nodes, &approvers).collect::<Vec<&ID>>(), vec![&19]);
}

