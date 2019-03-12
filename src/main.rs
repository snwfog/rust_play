// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;
extern crate rayon;

use rand::Rng;

fn main() {
  let mut rng = rand::thread_rng();
  let mut numbers = vec![0; 1_000_000];
  for i in 0..numbers.len() {
    numbers[i] = rng.gen_range(0, 1_000_000);
  }

//  println!("before {:?}", numbers);
  qsort(&mut numbers);
//  println!("after {:?}", numbers);
}

fn qsort(vec: &mut [i32]) {
  let mut rng = rand::thread_rng();
//  println!("length {}", vec.len());
  if vec.len() <= 1 {
    return;
  }

  let pivot = rng.gen_range(0, vec.len());
//  println!("pivot {}, low 0, high {}", pivot, vec.len());

  let mid = partition(vec, pivot as usize);
  let (lesser, greater_or_equal) = vec.split_at_mut(mid);
  rayon::join(|| qsort(lesser),
              || qsort(greater_or_equal));
}

fn partition(vec: &mut [i32], pivot: usize) -> usize {
  // Swap pivot with end
  let j = vec.len();
  let mut start = 0;
//  println!("before {}, {}", pivot, j - 1);
  vec.swap(pivot, j - 1);

  for i in 0..j - 1 {
    if vec[i] < vec[j - 1] {
      vec.swap(start, i);
      start += 1;
    }
  }

//  println!("after {}, {}", start, j - 1);
  vec.swap(start, j - 1);
  start + 1
}
