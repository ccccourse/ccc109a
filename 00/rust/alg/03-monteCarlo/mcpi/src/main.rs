use rand::Rng;

fn monte_carlo_pi(n:u32)->f64 {
  let mut rng = rand::thread_rng();
  let mut hits = 0;
  for _i in 0..n {
    let x:f64 = rng.gen();
    let y:f64 = rng.gen();
    if x*x+y*y <= 1.0 { hits += 1; }
  }
  return 4.0*(hits as f64/n as f64);
}


fn main() {
  let n = 1000000;
  println!("MonteCarloPi({})={}", n, monte_carlo_pi(n))
}
