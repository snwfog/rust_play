// The box is reference based
// and elements are created on the heap
// Since box also implement the Drop
// trait, value will also be cleaned up
// on the heap once the variable
// goes out of scope
use std::boxed::Box;

pub enum List {
  Cons(i32, Box<List>),
  Nil,
}
