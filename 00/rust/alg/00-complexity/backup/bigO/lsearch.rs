fn find(a:&[i32], o:i32)->i32 {
  let len = a.len();
  for i in 0..len-1 {
    if a[i] == o { return i as i32; }
  }
  return -1;
}

fn main() {
  let a = [1,5,2,4,3];
  println!("find([1, 5, 2, 4, 3], 2)={}", find(&a, 2));
}
