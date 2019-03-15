pub struct Counter {
  pub count: u32,
}

impl Counter {
  pub fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    let value = self.count;
    return if value > 5 {
      None
    } else {
      self.count += 1;
      Some(value)
    };
  }
}
