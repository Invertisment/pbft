use std::option::Option;

pub type ID = u64;
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
    view_id: ID,    // v
    seq_id: ID,     // n
    digest: Digest,  // d -- digest for m
    signature: Sig,  // sigma(p) -- sig of primary node
    message: TipMessage,    // m
}

#[derive(Debug)]
pub struct Prepare {
    view_id: ID,    // v
    seq_id: ID,     // n
    digest: Digest,  // d -- digest for m
    node_id: ID,    // i
    signature: Sig,  // sigma(i) -- Sig of sending node
}

#[derive(Debug)]
pub struct Commit {
    view_id: ID,    // v
    seq_id: ID,     // n
    digest: Digest,  // d -- digest for m
    node_id: ID,    // i
    signature: Sig,  // sigma(i) -- Sig of sending node
}

pub trait Request {
    fn get_view_id(&self) -> ID;   // v
    fn get_seq_id(&self) -> ID;    // n
    fn get_digest(&self) -> Digest; // n
}

pub trait NodeRequest: Request {
    fn get_node_id(&self) -> ID; // sigma(p) -- sig of primary node
}

impl PrePrepare {
    pub fn new(
        view_id: ID,    // v
        seq_id: ID,     // n
        digest: Digest,  // d -- digest for m
        signature: Sig,  // sigma(p) -- sig of primary node
        message: TipMessage,    // m
    ) -> PrePrepare {
        PrePrepare{
            view_id: view_id,    // v
            seq_id: seq_id,     // n
            digest: digest,  // d -- digest for m
            signature: signature,  // sigma(i) -- Sig of sending node
            message: message,    // m
        }
    }
    pub fn get_view(&self) -> ID {
        self.view_id
    }
    pub fn get_seq(&self) -> ID {
        self.seq_id
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
}

impl Commit {
    pub fn new(
        view_id: ID,    // v
        seq_id: ID,     // n
        digest: Digest,  // d -- digest for m
        node_id: ID,    // i
        signature: Sig,  // sigma(i) -- Sig of sending node
    ) -> Commit {
        Commit{
            view_id: view_id,    // v
            seq_id: seq_id,     // n
            digest: digest,  // d -- digest for m
            node_id: node_id,    // i
            signature: signature,  // sigma(i) -- Sig of sending node
        }
    }
}

impl Request for Commit {
    fn get_view_id(&self) -> ID {
        self.view_id
    }
    fn get_seq_id(&self) -> ID {
        self.seq_id
    }
    fn get_digest(&self) -> Digest {
        self.digest.clone()
    }
}

impl NodeRequest for Commit {
    fn get_node_id(&self) -> ID {
        self.node_id
    }
}

#[derive(Debug)]
pub struct Shutdown {}
