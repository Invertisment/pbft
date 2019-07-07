
pub type ID = u64;
pub type NodeID = ID;
pub type Sig = ID; // Signature. ID of the node that signed it. invalid ID -> nobody signed it.
pub type Digest = String; // Hash of something
pub type Tip = String; // current progress of Nodes

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
    message: Tip,    // m
    sender_id: NodeID,    // i // Not present in the original protocol
}

#[derive(Debug)]
pub struct Prepare {
    view_id: ID,    // v
    seq_id: ID,     // n
    digest: Digest,  // d -- digest for m
    sender_id: NodeID,    // i
    signature: Sig,  // sigma(i) -- Sig of sending node
}

#[derive(Debug)]
pub struct Commit {
    view_id: ID,    // v
    seq_id: ID,     // n
    digest: Digest,  // d -- digest for m
    sender_id: NodeID,    // i
    signature: Sig,  // sigma(i) -- Sig of sending node
}

pub trait NodeRequest {
    fn get_view_id(&self) -> ID;   // v
    fn get_seq_id(&self) -> ID;    // n
    fn get_digest(&self) -> Digest; // n
    fn get_sender_id(&self) -> ID; // sigma(p) -- sig of primary node
}

impl PrePrepare {
    pub fn new(
        view_id: ID,    // v
        seq_id: ID,     // n
        digest: Digest,  // d -- digest for m
        signature: Sig,  // sigma(p) -- sig of primary node
        message: Tip,    // m
        sender_id: NodeID,
    ) -> PrePrepare {
        PrePrepare{
            view_id: view_id,    // v
            seq_id: seq_id,     // n
            digest: digest,  // d -- digest for m
            signature: signature,  // sigma(i) -- Sig of sending node
            message: message,    // m
            sender_id: sender_id,
        }
    }
    pub fn get_message(&self) -> Tip {
        self.message.clone()
    }
    pub fn make_prepare(&self, sender_id: NodeID, sender_digest: String) -> Prepare {
        Prepare::new(
            self.view_id,
            self.seq_id,
            sender_digest,
            sender_id,
            sender_id
        )
    }
}

impl Prepare {
    pub fn new(
        view_id: ID,    // v
        seq_id: ID,     // n
        digest: Digest,  // d -- digest for m
        sender_id: NodeID,    // i
        signature: Sig,  // sigma(i) -- Sig of sending node
    ) -> Prepare {
        Prepare{
            view_id: view_id,    // v
            seq_id: seq_id,     // n
            digest: digest,  // d -- digest for m
            sender_id: sender_id,
            signature: signature,  // sigma(i) -- Sig of sending node
        }
    }
    pub fn make_commit(&self, sender_id: NodeID, sender_digest: String) -> Commit {
        Commit::new(
            self.view_id,
            self.seq_id,
            sender_digest,
            sender_id,
            sender_id
        )
    }
}

impl Commit {
    pub fn new(
        view_id: ID,    // v
        seq_id: ID,     // n
        digest: Digest,  // d -- digest for m
        sender_id: NodeID,    // i
        signature: Sig,  // sigma(i) -- Sig of sending node
    ) -> Commit {
        Commit{
            view_id: view_id,    // v
            seq_id: seq_id,     // n
            digest: digest,  // d -- digest for m
            sender_id: sender_id,    // i
            signature: signature,  // sigma(i) -- Sig of sending node
        }
    }
}

impl NodeRequest for Commit {
    fn get_view_id(&self) -> ID {
        self.view_id
    }
    fn get_seq_id(&self) -> ID {
        self.seq_id
    }
    fn get_digest(&self) -> Digest {
        self.digest.clone()
    }
    fn get_sender_id(&self) -> NodeID {
        self.sender_id
    }
}

impl NodeRequest for PrePrepare {
    fn get_view_id(&self) -> ID {
        self.view_id
    }
    fn get_seq_id(&self) -> ID {
        self.seq_id
    }
    fn get_digest(&self) -> Digest {
        self.digest.clone()
    }
    fn get_sender_id(&self) -> NodeID {
        self.sender_id
    }
}

impl NodeRequest for Prepare {
    fn get_view_id(&self) -> ID {
        self.view_id
    }
    fn get_seq_id(&self) -> ID {
        self.seq_id
    }
    fn get_digest(&self) -> Digest {
        self.digest.clone()
    }
    fn get_sender_id(&self) -> NodeID {
        self.sender_id
    }
}

#[derive(Debug)]
pub struct Shutdown {}
