// An attribute to hide warnings for unused code.
#![allow(dead_code)]

mod node;
mod dto;
mod network;
mod network_test;
use network::Network;
use crate::dto::{ID,Commit,Num};
use crate::node::{Message};

fn print_statuses(_net: &Network) {
    println!("----- Statuses: ------");
    _net.statuses.iter().for_each(|i| println!("{:?}", i));
    println!("----------------------");
}

fn print_queue(_net: &Network) {
    println!("------- Queue: -------");
    _net.queue.iter().for_each(|i| println!("{:?}", i));
    println!("----------------------");
}

fn queue_requests(net: &mut Network) {
    for i in 0..5 {
        net.queue_add(
            Message::commit(
                100,
                i as ID,
                Commit::new(1, 1, String::from(format!("digest {}", i)), i as Num, String::from(format!("signature {}", i)))));
    }
}

fn main() {
    println!("Hello, world!");
    let mut net = Network::new(5, 5);
    queue_requests(&mut net);
    for _i in 0..5 {
        print_queue(&net);
        print_statuses(&net);
        let _res = net.tick();
    }
}
