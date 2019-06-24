use std::sync::{Arc,RwLock};
use std::collections::{HashMap,HashSet};
use std::option::Option;
use crate::dto::{ID,Digest,NodeRequest};
use crate::util::{ensure_hm_val,convert_err};

pub type ViewID = ID;
pub type SeqID = ID;
pub type NodeID = ID;

#[derive(Debug)]
pub struct RequestTable<M: NodeRequest> {
    reqs: HashMap<SeqID, HashMap<ViewID, HashMap<Digest, HashMap<NodeID, Arc<RwLock<M>>>>>>,
}

impl <M>RequestTable<M> where M: NodeRequest {
    pub fn new() -> RequestTable<M> {
        RequestTable{
            reqs: HashMap::new()
        }
    }

    fn find_approvers<'a>(&self, ri: Arc<RwLock<M>>, nodes: &'a HashSet<ID>) -> Result<Vec<&'a ID>, String> {
        convert_err(ri.read()).map(|m| {
            match self.get_approvers(&*m) {
                Some(approvers) =>
                    nodes.iter().filter(|node_id| approvers.get(&node_id).is_some()).collect(),
                None => Vec::new(),
            }
        })
    }

    fn find_approver_count(&self, ri: Arc<RwLock<M>>, nodes: &HashSet<ID>) -> Result<usize, String> {
        self.find_approvers(ri, nodes)
            .map(|approvers| {
                let approvers: Vec<&ID> = approvers;
                approvers.iter().filter(|a| nodes.get(a).is_some()).count()
            })
    }

    pub fn is_sufficient(&self, ri: Arc<RwLock<M>>, nodes: &HashSet<ID>) -> Result<bool, String> {
        self.find_approver_count(ri, nodes)
            .map(|count| count > (((nodes.len()) * 2) / 3))
    }

    pub fn append(&mut self, rwarc: Arc<RwLock<M>>) -> Result<(),String> {
        convert_err(rwarc.read()).map(|_m| {
            let m: &M = &*_m;
            ensure_hm_val(&mut self.reqs, m.get_seq_num(), || HashMap::new());
            let mut in_seq = self.reqs.get_mut(&m.get_seq_num()).unwrap();
            ensure_hm_val(&mut in_seq, m.get_view_num(), || HashMap::new());
            let mut in_view = in_seq.get_mut(&m.get_view_num()).unwrap();
            ensure_hm_val(&mut in_view, m.get_digest(), || HashMap::new());
            let in_digest: &mut HashMap<NodeID, Arc<RwLock<M>>> = in_view.get_mut(&m.get_digest()).unwrap();
            in_digest.insert(m.get_node_id(), rwarc.clone());
        })
    }

    fn get_approvers(&self, ri: &M) -> Option<&HashMap<ID, Arc<RwLock<M>>>> {
        self.reqs.get(&ri.get_seq_num())
            .map(|views| views.get(&ri.get_view_num()))
            .unwrap_or(Option::None)
            .map(|digests| digests.get(&ri.get_digest()))
            .unwrap_or(Option::None)
    }

    fn get_by_arc(&self, rw: Arc<RwLock<M>>) -> Result<Option<&HashMap<ID, Arc<RwLock<M>>>>, String> {
        convert_err(rw.read()).map(|m| self.get_approvers(&*m))
    }
}

#[test]
fn get_by_arc_hit() {
    use crate::dto::{Commit};
    let ppre = Commit::new(
        400,
        400,
        "Digest".to_owned(),
        400,
        400);
    let arc = Arc::new(RwLock::new(ppre));
    let mut rt: RequestTable<Commit> = RequestTable::new();
    let append_res = rt.append(arc.clone());
    assert_eq!(append_res.is_ok(), true);
    let get_res = rt.get_by_arc(arc.clone());
    assert_eq!(get_res.is_ok(), true);
    let get_unwrap = get_res.unwrap();
    assert_eq!(get_unwrap.is_some(), true);
    assert_eq!(get_unwrap.unwrap().len(), 1);
}

#[test]
fn get_by_arc_miss() {
    use crate::dto::{Commit};
    let ppre = Commit::new(
        400,
        400,
        "Digest".to_owned(),
        400,
        400);
    let arc = Arc::new(RwLock::new(ppre));
    let rt: RequestTable<Commit> = RequestTable::new();
    let result = rt.get_by_arc(arc.clone());
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().is_none(), true);
}

#[test]
fn find_approver_count() {
    use crate::dto::{Commit};
    let ppre = Commit::new(
        2,
        2,
        "Digest".to_owned(),
        2,
        2);
    let arc = Arc::new(RwLock::new(ppre));
    let mut rt: RequestTable<Commit> = RequestTable::new();
    let res = rt.append(arc.clone());
    assert_eq!(res.is_ok(), true);
    let approver_count_res = rt.find_approver_count(arc, &(0..5 as ID).into_iter().collect());
    assert_eq!(approver_count_res.is_ok(), true);
    assert_eq!(approver_count_res.unwrap(), 1);
}
