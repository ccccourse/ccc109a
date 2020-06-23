fn f1(x:f64)->f64 {
    return 3.0/x;
}

fn f2(x:f64)->f64 {
    return x - 1.0 / 4.0 * (x * x - 3.0)
}

fn f3(x:f64)->f64 {
    return 1.0 / 2.0 * (x + 3.0 / x)
}

fn main() {
    let mut x1=1.0;
    let mut x2=1.0;
    let mut x3=1.0;
    for _i in 0..20 {
      x1 = f1(x1);  x2 = f2(x2); x3 = f3(x3);
      println!("x1:{} x2:{} x3:{}", x1, x2, x3);
    }
}
