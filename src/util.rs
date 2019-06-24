use crate::dto::{ID};
use std::collections::{HashSet,HashMap};
use std::ops::FnOnce;

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

pub fn ensure_hm_val<K, V, F>(top_level: &mut HashMap<K, V>, key_top: K, new_vt: F)
where K: std::cmp::Eq + std::hash::Hash,
      V: std::marker::Sized,
      F: FnOnce() -> V {
    if !top_level.contains_key(&key_top) {
        top_level.insert(key_top, new_vt());
    }
}

pub fn convert_err<Any, D>(res: Result<Any, D>) -> Result<Any, String>
where D: std::fmt::Debug {
    res.map_err(|e| format!("[err] {:?}", e))
}
