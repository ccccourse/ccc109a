const SEED_MAX = 9999997
var seed = d.getTime()%SEED_MAX; //371

fn random()->f64 {
    seed = (seed+37 ) % SEED_MAX
    let x = (seed as f64).sin() * 93177
    return x - x.floor()
}

module.exports = random