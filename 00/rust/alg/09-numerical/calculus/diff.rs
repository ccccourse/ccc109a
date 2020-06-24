use std::f64::consts::PI;

const D:f64 = 0.000001;

fn diff(f:fn(f64)->f64, x:f64)->f64 {
  return (f(x + D) - f(x - D)) / (D + D);
}

fn main() {
   println!("diff(sin(x), pi/3) = {}", diff(|x:f64|->f64 { x.sin() }, PI/3.0));
   println!("cos(pi/3) = {}", (PI/3.0).cos());
}
