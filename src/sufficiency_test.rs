#[cfg(test)]
mod reqtable_test {
    use crate::test_util::{new_nodes,new_nodes_vec};
    use crate::sufficiency::{one,two_thirds};
    #[test]
    fn test_approval_of_two_thirds_61() {
        // |R| = 3f + 1
        // accepted = 2f + 1
        // f = (|R| - 1) / 3
        // f_61 = (61 - 1) / 3 = 20
        // accepted_61 = 2 * f_61 + 1 = 2 * 20 + 1 = 41
        // At least 41 nodes of 61 have to accept to have a valid outcome
        let node_count = 61;
        let node_approver_count = 41;
        let nodes = new_nodes(node_count);
        for progress_below in 0..node_approver_count {
            println!("two_thirds_threshold {:?}/{:?} should be invalid", progress_below, node_count);
            assert_eq!(two_thirds(&nodes, &new_nodes_vec(progress_below)), false);
        }
        for progress_above in node_approver_count..node_count {
            println!("two_thirds_threshold {:?}/{:?} should be valid", progress_above, node_count);
            assert_eq!(two_thirds(&nodes, &new_nodes_vec(progress_above)), true);
        }
    }

    #[test]
    fn test_approval_of_two_thirds_61_should_avoid_noise() {
        // 0th element is deleted from node list and it's regarded to as noise
        let node_count = 61;
        let node_approver_count = 41;
        let mut nodes = new_nodes(node_count + 1);
        nodes.remove(&0);
        for progress_below in 0..node_approver_count+1 {
            println!("two_thirds_threshold {:?}/{:?} should be invalid", progress_below, node_count);
            assert_eq!(two_thirds(&nodes, &new_nodes_vec(progress_below)), false);
        }
        for progress_above in node_approver_count+1..node_count+1 {
            println!("two_thirds_threshold {:?}/{:?} should be valid", progress_above, node_count);
            assert_eq!(two_thirds(&nodes, &new_nodes_vec(progress_above)), true);
        }
    }

    #[test]
    fn approval_of_at_least_one_positive() {
        let nodes = new_nodes(20);
        let approvers = vec![5];
        assert_eq!(one(&nodes, &approvers), true);
    }

    #[test]
    fn test_approval_of_at_least_one_negative() {
        let nodes = new_nodes(20);
        let approvers = vec![100];
        assert_eq!(one(&nodes, &approvers), false);
    }

    #[test]
    fn test_approval_of_at_least_one_noise_no_approval() {
        let nodes = new_nodes(20);
        let approvers = vec![101, 102, 103, 105];
        assert_eq!(one(&nodes, &approvers), false);
    }

    #[test]
    fn test_approval_of_at_least_one_noise() {
        let nodes = new_nodes(20);
        let approvers = vec![101, 102, 103, 105, 19];
        assert_eq!(one(&nodes, &approvers), true);
    }
}
