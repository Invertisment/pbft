use std::option::Option;

pub type ID = u64;
pub type Num = i64;
pub type Sig = ID; // Signature. ID of the node that signed it. invalid ID -> nobody signed it.
pub type Digest = String; // Hash of something
pub type TipMessage = String; // current progress of Nodes
pub type Tip = Option<TipMessage>; // current progress of Nodes

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
    message: TipMessage,    // m
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

#[derive(Debug)]
pub struct RequestIdentifier {
    view_num: ID,    // v
    seq_num: ID,     // n
}

impl PrePrepare {
    pub fn new(
        view_num: ID,    // v
        seq_num: ID,     // n
        digest: Digest,  // d -- digest for m
        signature: Sig,  // sigma(p) -- sig of primary node
        message: TipMessage,    // m
    ) -> PrePrepare {
        PrePrepare{
            view_num: view_num,    // v
            seq_num: seq_num,     // n
            digest: digest,  // d -- digest for m
            signature: signature,  // sigma(i) -- Sig of sending node
            message: message,    // m
        }
    }
    pub fn get_view(&self) -> ID {
        self.view_num
    }
    pub fn get_seq(&self) -> ID {
        self.seq_num
    }
    pub fn get_digest(&self) -> &Digest {
        &self.digest
    }
    pub fn get_signature(&self) -> Sig {
        self.signature
    }
    pub fn get_message(&self) -> &TipMessage {
        &self.message
    }
    pub fn get_id(&self) -> RequestIdentifier {
        RequestIdentifier{
            view_num: self.view_num,
            seq_num: self.seq_num,
        }
    }
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
