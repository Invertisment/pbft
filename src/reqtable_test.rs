
#[cfg(test)]
mod reqtable_test {
    use std::sync::{RwLock,Arc};
    use crate::dto::{Commit,ID};
    use crate::reqtable::{RequestTable};
    use crate::test_util::{new_req,new_nodes};
    use crate::sufficiency::two_thirds;

    #[test]
    fn is_sufficient_empty() {
        let pp: RequestTable<Commit> = RequestTable::new(two_thirds);
        assert_eq!(pp.is_sufficient(&new_req(10, 15, 1), &new_nodes(0)), false);
    }

    struct ConfirmationType {
        view: ID,
        seq: ID,
        count: usize,
    }

    fn add_confirmations_complex(table: &mut RequestTable<Commit>, types: Vec<ConfirmationType>) {
        for t in types {
            add_confirmations(table, t.view, t.seq, t.count)
        }
    }

    fn add_confirmations<'a>(table: &'a mut RequestTable<Commit>, view: ID, seq: ID, count: usize) {
        for i in 0..count as ID {
            let res = table.append(Arc::new(RwLock::new(new_req(view, seq, i))));
            if res.is_err() {
                panic!(res)
            }
        }
    }

    #[test]
    fn is_sufficient_threshold() {
        // |R| = 3f + 1
        // accepted = 2f + 1
        // f = (|R| - 1) / 3
        // f_61 = (61 - 1) / 3 = 20
        // accepted_61 = 2 * f_61 + 1 = 2 * 20 + 1 = 41
        // At least 41 nodes of 61 have to accept to have a valid outcome
        let test_req = new_req(0, 0, 0);
        let node_count = 61;
        let nodes = new_nodes(node_count);
        for progress_below in 0..41 {
            let mut confirm_progress = RequestTable::new(two_thirds);
            add_confirmations(&mut confirm_progress, 0, 0, progress_below);
            println!("is_sufficient_threshold below {:?}/{:?}", progress_below, node_count);
            assert_eq!(confirm_progress.is_sufficient(&test_req, &nodes), false);
        }
        for progress_over in (41..(node_count + 1)).into_iter().rev() {
            let mut confirm_progress = RequestTable::new(two_thirds);
            add_confirmations(&mut confirm_progress, 0, 0, progress_over);
            println!("is_sufficient_threshold over {:?}/{:?}", progress_over, node_count);
            assert_eq!(confirm_progress.is_sufficient(&test_req, &nodes), true);
        }
    }

    #[test]
    fn is_sufficient_threshold_two_progresses() {
        // |R| = 3f + 1
        // accepted = 2f + 1
        // f = (|R| - 1) / 3
        // f_61 = (61 - 1) / 3 = 20
        // accepted_61 = 2 * f_61 + 1 = 2 * 20 + 1 = 41
        // At least 41 nodes of 61 have to accept to have a valid outcome
        let test_req0 = new_req(0, 0, 0);
        let test_req1 = new_req(0, 1, 0);
        let node_count = 61;
        let nodes = new_nodes(node_count);
        for progress_below in 10..41 {
            let mut confirm_progress = RequestTable::new(two_thirds);
            add_confirmations_complex(
                &mut confirm_progress,
                vec![
                    ConfirmationType{view: 0, seq: 0, count: progress_below},
                    ConfirmationType{view: 0, seq: 1, count: progress_below - 10}
                ]);
            println!("is_sufficient_threshold below {:?}/{:?}", progress_below, node_count);
            assert_eq!(confirm_progress.is_sufficient(&test_req0, &nodes), false);
            assert_eq!(confirm_progress.is_sufficient(&test_req1, &nodes), false);
        }
        for progress_over in (41..(node_count + 1)).into_iter().rev() {
            let mut confirm_progress = RequestTable::new(two_thirds);
            add_confirmations_complex(
                &mut confirm_progress,
                vec![
                    ConfirmationType{view: 0, seq: 0, count: progress_over},
                    ConfirmationType{view: 0, seq: 1, count: progress_over - 40}
                ]);
            println!("is_sufficient_threshold over {:?}/{:?}", progress_over, node_count);
            assert_eq!(confirm_progress.is_sufficient(&test_req0, &nodes), true);
            assert_eq!(confirm_progress.is_sufficient(&test_req1, &nodes), false);
        }
    }


}
