#[cfg(test)]
mod network_interaction_test {
    use crate::test_util::{new_random_preprepare,new_nodes};
    use crate::node::Message;
    use crate::dto::ID;
    use std::sync::{Arc,RwLock};

    #[test]
    fn multi_should_produce_copies_for_multiple_nodes() {
        let pp = Arc::new(RwLock::new(new_random_preprepare()));
        let sender_id = 15 as ID;
        let node_ids = new_nodes(20);
        let multi: Vec<Message> = Message::multiply(Message::preprepare, pp, sender_id, &node_ids).collect();
        assert_eq!(multi.len(), 20)
    }
}
