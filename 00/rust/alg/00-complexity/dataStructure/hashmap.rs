use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert("a", 1);
  map.insert("b", 2);
  map.insert("c", 3);
  
  for (key, val) in map.iter() {
      println!("key: {} val: {}", key, val);
  }
}

