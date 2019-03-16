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
use std::io::Write;
use std::{env, fs::File, process};

fn main() -> std::io::Result<()> {
  // chap1::q1::is_all_unique("hahaha")
  // let closure = |num| println!("hahaha");

  // let args: Vec<String> = env::args().collect();
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("problem parsing arguments: {}", err);
    process::exit(1);
  });

  // open_da_file(&config.filename)?;
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
  Ok(())
}

fn open_da_file_and_write(file_name: &String) -> std::io::Result<()> {
  // Ok(match foo.bar() {
  //   Ok(result) => result,
  //   Err(err) => return Err(From::from(err)),
  // })
  let mut file = File::create(file_name)?;
  file.write_all(b"hello world")
  // .write_all(b"Hello world")
  // File::create("haha.txt")?.write_all(b"Hello world");
}

mod cons;
use cons::List::{Cons, Nil};

fn list() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// fn open_da_file(file_name: &String) -> std::io::Result<()> {
//   let h = "hello".to_owned();

//   do {
//     mut f <- File::create(file_name);
//     s <- Ok(h + " world!");
//     f.write_all(s)
//   };
// }
