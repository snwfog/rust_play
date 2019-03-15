#[derive(PartialEq, Debug)]
pub struct Shoe {
  pub size:  u32,
  pub style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

pub fn longest<'a>(_x: &str, _y: &str) -> String {
  let result = String::from("really long string");
  result
}
