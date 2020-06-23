use rand::Rng;

fn monteCarloPi(n:f64) {
  let mut rng = rand::thread_rng();
  let hits = 0;
  for _i in 0..n {
    let x = rng.gen();
    let y = rng.gen();
    if x*x+y*y <= 1.0 { hits += 1; }
  }
  return 4.0*(hits/n);
}


fn main() {
  let n = 1000000;
  println!("MonteCarloPi({})={}", n, monteCarloPi(n))
}


