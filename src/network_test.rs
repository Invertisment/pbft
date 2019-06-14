
#[cfg(test)]
mod network_test {

    use crate::dto::{ID,Commit,Num};
    use crate::node::{Message};
    use crate::network::{Network};

    fn send_requests(net: &mut Network, count: usize) {
        for i in 0..count {
        net.queue_add(
            Message::commit(
                100,
                i as ID,
                Commit::new(1, 1, String::from(format!("digest {}", i)), i as Num, String::from(format!("signature {}", i)))));
        }
    }

    fn mk_net() -> Network {
        Network::new(5, 10)
    }

    #[test]
    fn network_empty() {
        let mut net: Network = Network::new(0, 10);
        send_requests(&mut net, 5);
        match net.tick() {
            Ok(_res) => panic!("Should fail with no nodes"),
            Err(e) => assert_eq!(e, "No nodes were found"),
        };
    }

    #[test]
    fn network_no_reqs() {
        let mut net: Network = mk_net();
        match net.tick() {
            Ok(_res) => panic!("Should fail with no requests"),
            Err(e) => assert_eq!(e, "No more requests"),
        };
    }

    #[test]
    fn network_drops_packets() {
        let mut net: Network = mk_net();
        send_requests(&mut net, 5);
        match net.tick() {
            Ok(res) => assert!(res),
            Err(e) => panic!(e),
        };
    }

    #[test]
    fn network_count_packets() {
        let mut net: Network = mk_net();
        send_requests(&mut net, 5);
        let mut res : i64 = 0;
        loop {
            match net.tick() {
                Ok(b) => if b {
                    res = &res + 1
                },
                Err(_e) => break,
            };
        }
        assert_eq!(res, 5)
    }

    #[test]
    fn node_remove() {
        let mut net = Network::new(2, 5);
        match net.remove_node(1).unwrap().join() {
            Ok(_) => {},
            Err(e) => panic!(e),
        };
        send_requests(&mut net, 5);
        match net.tick() {
            Ok(res) => assert!(res),
            Err(e) => panic!(e),
        };
        match net.tick() {
            Ok(res) => assert!(!res),
            Err(e) => panic!(e),
        };
    }

}
