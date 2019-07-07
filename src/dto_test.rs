#[cfg(test)]
mod dto_transformation_test {
    use crate::dto::{ID,NodeRequest};
    use crate::test_util::new_random_preprepare;

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
}
