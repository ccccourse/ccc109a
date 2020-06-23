use std::env;
use std::collections::HashMap;

fn main() {
    let mut e2c = HashMap::new();
    e2c.insert("a".to_string(), "一隻".to_string());
    e2c.insert("dog".to_string(), "狗".to_string());
    let args: Vec<String> = env::args().collect();
    println!("{:?}.", args);
    for e in &args[1..] {
      let c = e2c.get(&e.to_string());
      print!("{}={:?} ", e, c);
    }
}
