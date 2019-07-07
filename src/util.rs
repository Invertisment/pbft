use crate::dto::{ID};
use std::collections::HashMap;
use std::ops::FnOnce;
use std::sync::{Arc,RwLock};

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

pub fn find_others<'a>(me: ID, all_nodes: impl Iterator<Item = &'a ID> + 'a) -> impl Iterator<Item = ID> + 'a {
    all_nodes.filter(move |other| **other != me).map(|i| *i)
}

#[test]
fn should_find_others() {
    let set: Vec<ID> = [1, 5, 56, 12, 214, 11].iter().map(|i| *i).collect();
    let others_5: Vec<ID> = find_others(5, set.iter()).collect();
    assert_eq!(others_5, vec![1, 56, 12, 214, 11]);
    let others_214: Vec<ID> = find_others(214, set.iter()).collect();
    assert_eq!(others_214, vec![1, 5, 56, 12, 11])
}

pub fn digest(id: ID) -> String {
    id.to_string()
}

pub fn wrap_to_arc_option<T>(t: T) -> Option<Arc<RwLock<T>>> {
    Option::Some(Arc::new(RwLock::new(t)))
}
