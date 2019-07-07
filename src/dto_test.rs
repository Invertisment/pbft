use crate::dto::{PrePrepare,ID,NodeRequest};
use std::time::Instant;

fn random() -> ID {
    Instant::now().elapsed().as_secs() as ID
}

fn new_random_preprepare() -> PrePrepare {
    let sender_id = random();
    PrePrepare::new(
        random(),    // v
        random(),     // n
        "sample text".to_owned(),  // d -- digest for m
        sender_id,  // sigma(p) -- sig of primary node
        "Tip message".to_owned(),    // m
        sender_id,
    )
}

#[test]
fn preprepare_should_create_prepare() {
    let pp = new_random_preprepare();
    let new_sender_id = 1337 as ID;
    let new_sender_digest = "2448".to_owned();
    let p = pp.make_prepare(new_sender_id, new_sender_digest.clone());
    assert_eq!(p.get_view_id(), pp.get_view_id());
    assert_eq!(p.get_seq_id(), pp.get_seq_id());
    assert_eq!(p.get_digest(), new_sender_digest);
    assert_eq!(p.get_sender_id(), new_sender_id);
}
