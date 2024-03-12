fn index(idx: usize, arr: &[u8]) -> Option<u8> {
  if idx < arr.len() {
      unsafe {
          Some(*arr.get_unchecked(idx))
      }
  } else {
      None
  }
}

fn main() {
  let arr = [1,2,3,4];

  let r = index(2, &arr);

  println!("value: {:?}", r);

}