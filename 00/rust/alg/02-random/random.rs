use std::time::{SystemTime, UNIX_EPOCH};

const SEED_MAX:u64 = 9999997;
static mut SEED:u64 = 0;

fn random()->f64 {
  unsafe {
    SEED = (SEED+37 ) % SEED_MAX;
    let x = (SEED as f64).sin() * 93177.0;
    return x - x.floor();
  }
}

fn main() {
  unsafe {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    SEED = since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
  }
  for _i in 1..10 {
    println!("{}", random());
  }
}
