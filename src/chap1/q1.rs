pub fn is_all_unique(s: &str) {
  let mut chars = [ 26 * 2 ];

  for c in s.chars() {
    let v = c.to_digit(10);
    println!("{:?}, {:?}", c, v);
    println!("{}", match v {
      Some(int) => int,
      None => {
        0
      },
    });
  }
}
