fn f(x:f64)->f64 {
  return x*x-4.0*x+1.0;
}

fn bsolve(f:fn(f64) -> f64,a:f64,b:f64)->f64 {
  let c = (a+b)/2.0;
  if (a-b).abs() < 0.00001 { return c; }
  if f(c)*f(a)>=0.0 {
    return bsolve(f, c, b);
  } else {
    return bsolve(f, a, c);
  }
}

fn main() {
  let x=bsolve(f, 0.0, 1.0);
  println!("x={} f(x)={}", x, f(x));
}
