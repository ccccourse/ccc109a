use std::env;
use std::collections::HashMap;

fn main() {
    let e2c: HashMap<&str, &str> =
    [("dog", "狗"),
     ("cat", "貓"),
     ("a", "一隻")]
     .iter().cloned().collect();
    // use the values stored in map
    // Prints each argument on a separate line
    let args: Vec<String> = env::args().collect();
    println!("{:?}.", args);
    for e in &args[1..] {
      let c = e2c.get(&e.to_string());
      print!("{}={:?}", e, c);
      print!("{} ", e);
    }
    /*
    for e in &args[1..] {
      print!("{} ", e2c.get(e).to);
    }
    */
    /*
    for (_key, val) in c2e.iter() {
      print!("{}", val);
    }
    */
}
