
#[cfg(test)]
mod network_test {

    use crate::dto::{Num,Commit};
    use crate::node::{Node};
    use crate::network::{Network,Request};
    use std::collections::{HashMap,VecDeque};
    use std::boxed::Box;

    fn mk_nodes() -> HashMap<Num, Node>{ // TODO: is HashMap not a Map? :O
        let mut nodes: HashMap<Num, Node> = HashMap::new();
        nodes.insert(0, Node{round: 25});
        nodes.insert(1, Node{round: 25});
        return nodes;
    }

    fn mk_requests<'a>() -> VecDeque<Box<dyn Request>> {
        let mut queue: VecDeque<Box<Request>> = VecDeque::new();
        queue.push_back(Box::new(Commit::new(1, 1, String::from("digest 0"), 1, String::from("signature 0"))));
        queue.push_back(Box::new(Commit::new(1, 1, String::from("digest 1"), 1, String::from("signature 1"))));
        queue.push_back(Box::new(Commit::new(1, 1, String::from("digest 2"), 2, String::from("signature 2"))));
        queue.push_back(Box::new(Commit::new(1, 1, String::from("digest 3"), 3, String::from("signature 3"))));
        queue.insert(2, Box::new(Commit::new(1, 1, String::from("digest 4"), 4, String::from("signature 4"))));
        return queue;
    }

    fn mk_net() -> Network {
        Network::new(mk_nodes(), mk_requests())
    }

    #[test]
    fn network_drops_packets() {
        let mut net: Network = mk_net();
        let res : bool = net.tick();
        assert!(res)
    }

    #[test]
    fn network_count_packets() {
        let mut net: Network = mk_net();
        let mut res : i64 = 0;
        while net.tick() {
            res = &res + 1;
        }
        assert_eq!(res, 5)
    }

}
