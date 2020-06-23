use std::time::SystemTime;

fn fibonacci (n:i64) -> i64 {
  if n < 0 { return -1; }
  if n == 0 { return 0; }
  if n == 1 { return 1; }
  return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
  let start_time = SystemTime::now();
  println!("start_time:{:?}", start_time);
  println!("fibonacci(38)={}", fibonacci(38));
  let end_time = SystemTime::now();
  println!("end_time:{:?}", end_time);
  let duration = end_time.duration_since(start_time).expect("Clock may have gone backwards");
  println!("duration:{:?}", duration);
}
