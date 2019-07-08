#[cfg(test)]
mod dto_transformation_test {
    use crate::dto::{ID,NodeRequest};
    use crate::test_util::new_random_preprepare;

    #[test]
    fn preprepare_should_create_prepare() {
        let pp = new_random_preprepare();
        let new_sender_id = 1337 as ID;
        let p = pp.make_prepare(new_sender_id);
        assert_eq!(p.get_view_id(), pp.get_view_id());
        assert_eq!(p.get_seq_id(), pp.get_seq_id());
        assert_eq!(p.get_digest(), "digest".to_owned());
        assert_eq!(p.get_sender_id(), new_sender_id);
    }
}
