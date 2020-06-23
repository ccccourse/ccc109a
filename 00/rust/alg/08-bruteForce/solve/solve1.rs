fn f(x:f64)->f64 {
  return x*x-4.0*x+1.0;
  // return (x*x+2.0*x).sin();
}

fn main() {
  let mut x:f64 = -100.0;
  while x < 100.0 {
    if f(x).abs() < 0.001 {
      println!("x={} f(x)={}", x, f(x));
    }
    x += 0.001;
  }
}
