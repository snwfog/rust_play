// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;
extern crate rayon;

mod qsort;
mod chap1;

fn main() {
  chap1::q1::is_all_unique("hahaha")
}

fn longest<'a>(_x: &str, _y: &str) -> String {
  let result = String::from("really long string");
  result
}
