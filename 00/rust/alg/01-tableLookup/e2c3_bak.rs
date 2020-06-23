use std::env;
use std::collections::HashMap;

fn dput(map:&HashMap<&str, &str>, k:String, v:String) {
  map.insert(&k.to_string(), &v.to_string());
}

fn main() {
    let mut e2c = HashMap::new();
    dput(&e2c, "a".to_string(), "一隻".to_string());
    dput(&e2c, "dog".to_string(), "狗".to_string());
    let args: Vec<String> = env::args().collect();
    println!("{:?}.", args);
    for e in &args[1..] {
      let c = e2c.get(&e.to_string());
      print!("{}={:?} ", e, c);
    }
}
