// An attribute to hide warnings for unused code.
#![allow(dead_code)]

mod node;
mod dto;
mod network;
mod network_test;

//use crate::dto::{Num,Commit};
//use crate::node::{Node};
//use crate::network::{Network,Request};
//use std::collections::{HashMap,VecDeque};
//use std::boxed::Box;

fn main() {
    println!("Hello, world!");
    //let some_number: dto::Num = 155;
    //println!("{:?}", some_number);
    //let commit : dto::Commit = dto::Commit::new(14, 14, String::from("digest"), 14, String::from("signature"));
    //println!("{:?}", commit);
    //let number: u64 = 111111111111111;
    //let b: u16 = number as u16;
    //println!("{:?}", b);
    ////let n: &node::TargetNode = node::Node{Round: 15} as &mut node::TargetNode;
    ////println!("{:?}", n.sendCommit(commit));

    //let net: &mut Network = &mut Network::new(mk_nodes(), mk_requests());
    //println!("{:?}", net);
    //loop {
    //    let res : bool = net.tick();
    //    println!("{:?}", net);
    //    if !res {
    //        break;
    //    }
    //}
}
