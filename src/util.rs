use crate::dto::{ID};
use std::collections::{HashSet};
use std::iter::{Iterator};

pub fn retain_others(to_remove: ID, set: &HashSet<ID>) -> HashSet<ID> {
    let mut cloned: HashSet<ID> = set.clone();
    cloned.remove(&to_remove);
    cloned
}

#[test]
fn should_copy_and_remove_own_id() {
    fn make_ids(size: usize) -> HashSet<ID> {
        (0..size as ID).into_iter().collect()
    }
    let set: &HashSet<ID> = &make_ids(10);
    let retained = retain_others(4 as ID, set);
    assert_eq!(retained.len(), 9);
    assert!(!retained.contains(&4));
}
