fn first_word(s: &str) -> &str {
  let data = s.as_bytes();

  for (i, &item) in data.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      } 
  }

  &s[..]
}


fn main() {
  let s = foo();

  let name = String::from("hello world");

  let r = first_word(&name[..]);

  println!("{s} {r}");
}