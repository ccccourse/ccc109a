use std::time::{SystemTime, UNIX_EPOCH};

const SEED_MAX:u64 = 9999997;

struct RandomGen {
    seed: u64 
}

impl RandomGen {
    pub fn new(seed: u64) -> Self {
        Self {
            seed: seed
        }   
    }   

    pub fn random(&mut self) -> f64 {
        self.seed = (self.seed + 37) % SEED_MAX;
        let x = (self.seed as f64).sin() * 93177.0;
        x - x.floor()
    }
}

fn main() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let seed = since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    let mut rndgen = RandomGen::new(seed);
    for _ in 1..10 {
        println!("{}", rndgen.random());
    }   
}
