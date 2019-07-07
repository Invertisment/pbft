// An attribute to hide warnings for unused code.
#![allow(dead_code)]

mod dto;
mod dto_test;
mod network;
mod network_test;
mod node;
mod node_test;
mod reqtable;
mod reqtable_test;
mod sufficiency;
mod sufficiency_test;
mod test_util;
mod util;
use network::Network;
use crate::dto::{PrePrepare};
use crate::node::{Message};
use std::sync::{Arc,RwLock};
use std::env;
use std::thread;
use std::time::Duration;

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
    let sender_id = 0;
    let target_id = 1;
    net.queue_add(
            Message::preprepare(
                sender_id,
                target_id,
                Arc::new(RwLock::new(PrePrepare::new(
                    0,
                    1,
                    util::digest(sender_id),
                    sender_id,
                    "Advanced tip message".to_owned(),
                    sender_id)))));

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
        println!("Ticking");
        net.queue_update();
        let _res = net.tick();
        thread::sleep(Duration::from_millis(100));
    }
}

fn interactive_mode() {
}
