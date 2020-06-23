const HMAX:u32 = 9999997;

fn hash(s:&str)->u32 {
  let mut h:u32 = 5381;
  for c in s.chars() {
    h = ((h * 33)%HMAX) ^ (c as u32);
  }
  return h;
}

fn main() {
  println!("hash(hello)={}", hash("hello"));
  println!("hash(hello!)={}", hash("hello!"));
}
