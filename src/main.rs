// An attribute to hide warnings for unused code.
#![allow(dead_code)]

mod dto;
mod network;
mod network_test;
mod node;
mod reqtable;
mod reqtable_test;
mod sufficiency;
mod sufficiency_test;
mod test_util;
mod util;
use network::Network;
use crate::dto::{ID,Commit};
use crate::node::{Message};
use std::env;

fn print_statuses(net: &Network) {
    println!("----- Statuses: ------");
    net.get_statuses().for_each(|(id, v)| {
        println!("{:?} {:?}", id, v);
    });
    println!("----------------------");
}

fn print_queue<'l>(net: &Network) {
    println!("------- Queue: -------");
    net.get_queue().for_each(|i| println!("{:?}", i));
    println!("----------------------");
}

fn queue_requests(net: &mut Network) {
    for i in 0..5 {
        net.queue_add(
            Message::commit(
                100,
                i as ID,
                Commit::new(1 as ID, 1 as ID, String::from(format!("digest {}", i)), i as ID, i as ID)));
    }
}

fn is_interactive_ui(args: &mut env::Args) -> bool {
    args.any(|arg| arg == "--ui")
}

fn main() {
    println!("Hello, world!");
    if is_interactive_ui(&mut env::args()) {
        interactive_mode();
        return
    }
    println!("To run with interactive UI add option '--ui'");
    let mut net = Network::new(5, 5);
    queue_requests(&mut net);
    for _i in 0..5 {
        print_queue(&net);
        print_statuses(&net);
        let _res = net.tick();
    }
}

fn interactive_mode() {
}
