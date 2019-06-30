use std::sync::{Arc,RwLock};
use std::collections::{HashMap,HashSet};
use std::option::Option;
use crate::dto::{ID,Digest,NodeRequest};
use crate::util::{ensure_hm_val,convert_err};
use crate::sufficiency::SufficiencyChecker;

pub type ViewID = ID;
pub type SeqID = ID;
pub type NodeID = ID;

pub struct RequestTable<M: NodeRequest> {
    reqs: HashMap<SeqID, HashMap<ViewID, HashMap<Digest, HashMap<NodeID, Arc<RwLock<M>>>>>>,
    check_sufficiency: SufficiencyChecker,
}

impl <M>RequestTable<M> where M: NodeRequest {
    pub fn new(sufficiency_fn: SufficiencyChecker) -> RequestTable<M> {
        RequestTable{
            reqs: HashMap::new(),
            check_sufficiency: sufficiency_fn,
        }
    }

    fn find_approvers<'a>(&self, ri: Arc<RwLock<M>>, nodes: &'a HashSet<ID>) -> Result<Vec<ID>, String> {
        convert_err(ri.read()).map(|m| {
            match self.get_approvers(&*m) {
                Some(approvers) =>
                    nodes.iter().filter(|node_id| approvers.get(&node_id).is_some()).map(|id| *id).collect(), // TODO: must return raw approvers; not from the node list
                None => Vec::new(),
            }
        })
    }

    pub fn is_sufficient(&self, ri: Arc<RwLock<M>>, all_nodes: &HashSet<ID>) -> Result<bool, String> {
        self.find_approvers(ri, all_nodes)
            .map(|approver_nodes| (&self.check_sufficiency)(all_nodes, &approver_nodes))
    }

    pub fn append(&mut self, rwarc: Arc<RwLock<M>>) -> Result<(),String> {
        convert_err(rwarc.read()).map(|_m| {
            let m: &M = &*_m;
            ensure_hm_val(&mut self.reqs, m.get_seq_id(), || HashMap::new());
            let mut in_seq = self.reqs.get_mut(&m.get_seq_id()).unwrap();
            ensure_hm_val(&mut in_seq, m.get_view_id(), || HashMap::new());
            let mut in_view = in_seq.get_mut(&m.get_view_id()).unwrap();
            ensure_hm_val(&mut in_view, m.get_digest(), || HashMap::new());
            let in_digest: &mut HashMap<NodeID, Arc<RwLock<M>>> = in_view.get_mut(&m.get_digest()).unwrap();
            in_digest.insert(m.get_node_id(), rwarc.clone());
        })
    }

    fn get_approvers(&self, ri: &M) -> Option<&HashMap<ID, Arc<RwLock<M>>>> {
        self.reqs.get(&ri.get_seq_id())
            .map(|views| views.get(&ri.get_view_id()))
            .unwrap_or(Option::None)
            .map(|digests| digests.get(&ri.get_digest()))
            .unwrap_or(Option::None)
    }

    fn get_by_arc(&self, rw: Arc<RwLock<M>>) -> Result<Option<&HashMap<ID, Arc<RwLock<M>>>>, String> {
        convert_err(rw.read()).map(|m| self.get_approvers(&*m))
    }
}

#[cfg(test)]
use crate::sufficiency::{two_thirds};

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
    let mut rt: RequestTable<Commit> = RequestTable::new(two_thirds);
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
    let rt: RequestTable<Commit> = RequestTable::new(two_thirds);
    let result = rt.get_by_arc(arc.clone());
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().is_none(), true);
}
