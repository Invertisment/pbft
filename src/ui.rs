use crate::network::Network;
use std::thread;
use std::time::Duration;
use std::io;
use std::sync::{RwLock,Arc};
use crate::dto::{PrePrepare};
use crate::node::{Message};

fn print_line() {
    println!("----------------------------------------------------------------------------------------------------");
}

fn readln() -> String {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    buffer
}

fn read_number() -> Option<u64> {
    readln().trim().parse::<u64>().ok()
}

fn print_menu() {
    print_line();
    println!("1. gather packets into queue");
    println!("2. propagate first packet from queue");
    println!("2a. propagate all packets from queue (not from channel)");
    println!("2b. propagate everything until channel is exhausted");
    println!("3. new PrePrepare request (from 1st node to 2nd)");
}

pub fn print_statuses(net: &Network) {
    println!("----- Statuses: ------");
    net.get_statuses().for_each(|(id, v)| {
        println!("{:?} {:?}", id, v);
    });
    println!("----------------------");
}

pub fn print_queue<'l>(net: &Network) {
    println!("------- Queue: -------");
    net.get_queue().for_each(|i| println!("{:?}", i));
    println!("----------------------");
}

pub fn new_preprepare<'l>(net: &mut Network) {
    println!("------- PrePrepare input: -------\n ");
    println!("Please enter seq_id:");
    let seq_id = read_number();
    if seq_id.is_none() {
        println!("Bad number. Ending");
        return;
    }
    println!("Please enter your message:");
    let message = readln().trim().to_owned();
    println!("Adding the request to queue");
    println!("---------------------------------");
    let sender_id = 0;
    // force (almost (bug: 0 -> 0 will filter it out)) all nodes to get the requests
    for m in Message::multiply(
        Message::preprepare,
        Arc::new(RwLock::new(PrePrepare::new(
            0,
            seq_id.unwrap(),
            "".to_owned(),
            1337,
            message,
            sender_id))),
        sender_id,
        &net.get_nodes()) {
        net.queue_add(m)
    }
}

pub fn interactive_mode(mut net: &mut Network) {
    loop {
        print_queue(&net);
        print_statuses(&net);
        print_menu();
        match readln().trim() {
            "1" => {
                net.queue_update();
            },
            "2" => {
                let _res = net.tick();
                thread::sleep(Duration::from_millis(100));
            },
            "2a" => {
                let _res = net.tick_queue_all();
                thread::sleep(Duration::from_millis(100));
            },
            "2b" => {
                let _res = net.tick_until_empty_skip_queue();
                thread::sleep(Duration::from_millis(100));
            },
            "3" => {
                new_preprepare(&mut net);
            }
            _ => println!("Unknown command")
        }
    }
}
