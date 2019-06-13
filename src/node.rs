
use crate::dto::{PrePrepare,Prepare,Commit,Num};

pub trait TargetNode {
    fn send_pre_prepare(&self, _req: PrePrepare) -> bool;
    fn send_prepare(&self, _req: Prepare) -> bool;
    fn send_commit(&self, _req: Commit) -> bool;
}

impl TargetNode for Node {
    fn send_pre_prepare(&self, _req: PrePrepare) -> bool{false}
    fn send_prepare(&self, _req: Prepare) -> bool{false}
    fn send_commit(&self, _req: Commit) -> bool{false}
}

#[derive(Debug)]
pub struct Node {
    pub round: Num,
}
