use std::option::Option;

pub type ID = u64;
pub type Num = i64;
pub type Sig = String; // Signature
pub type Digest = String; // Hash of something
pub type State = Option<String>; // current progress of Nodes

/*
Parameters:

p -- primary node of the view
i -- current node of the view
*/

#[derive(Debug)]
pub struct PrePrepare {
    view_num: ID,    // v
    seq_num: ID,     // n
    digest: Digest,  // d -- digest for m
    signature: Sig,  // sigma(p) -- sig of primary node
    message: Num,    // m
}

#[derive(Debug)]
pub struct Prepare {
    view_num: Num,    // v
    seq_num: Num,     // n
    digest: Digest,  // d -- digest for m
    node_num: Num,    // i
    signature: Sig,  // sigma(i) -- Sig of sending node
}

#[derive(Debug)]
pub struct Commit {
    view_num: Num,    // v
    seq_num: Num,     // n
    digest: Digest,  // d -- digest for m
    node_num: Num,    // i
    signature: Sig,  // sigma(i) -- Sig of sending node
}

impl Commit {
    pub fn new(
        view_num: Num,    // v
        seq_num: Num,     // n
        digest: Digest,  // d -- digest for m
        node_num: Num,    // i
        signature: Sig,  // sigma(i) -- Sig of sending node
    ) -> Commit {
        Commit{
            view_num: view_num,    // v
            seq_num: seq_num,     // n
            digest: digest,  // d -- digest for m
            node_num: node_num,    // i
            signature: signature,  // sigma(i) -- Sig of sending node
        }
    }
}

#[derive(Debug)]
pub struct Shutdown {}
