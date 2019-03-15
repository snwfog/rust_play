extern crate rayon;

mod chap1;
mod config;
mod counter;
mod qsort;
mod shoe;

// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;
use config::Config;
use counter::Counter;
use shoe::Shoe;
use std::collections::BinaryHeap;
use std::{env, process};

fn main() {
  // chap1::q1::is_all_unique("hahaha")
  // let closure = |num| println!("hahaha");

  // let args: Vec<String> = env::args().collect();
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("query: {}, filename: {}", config.query, config.filename);

  let mut heap = BinaryHeap::with_capacity(100);
  heap.push(1);
  heap.push(3);

  for val in heap.iter() {
    println!("{}", val);
  }

  let rack = vec![
    Shoe {
      size:  10,
      style: String::from("nike"),
    },
    Shoe {
      size:  20,
      style: String::from("adidas"),
    },
    Shoe {
      size:  30,
      style: String::from("vans"),
    },
  ];

  let fits = shoe::shoes_in_my_size(rack, 20);
  println!("{:?}", fits);
  // closure(23);

  let counter = Counter::new();
  println!("{}", counter.count);

  // for v in counter.iter() {
  //   println!("{}", v);
  // }
}
