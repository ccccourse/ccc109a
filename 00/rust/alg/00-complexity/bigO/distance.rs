#[derive(Debug)]
struct Point {
  x: f64,
  y: f64,
}

fn distance(p1:&Point, p2:&Point)->f64 {
  let dx = p1.x - p2.x;
  let dy = p1.y - p2.y;
  return dx*dx+dy*dy;
}

fn main() {
  let p1 = Point {x: 3.0, y:4.0};
  let p2 = Point {x: 0.0, y:0.0};
  println!("distance({:?},{:?})={}", p1, p2, distance(&p1, &p2));
}
