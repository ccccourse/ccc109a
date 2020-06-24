use ndarray::{arr2};
use ndarray::prelude::*;
use std::iter::FromIterator;

fn main() {
    let a = array![[1.,2.,3.], 
                   [4.,5.,6.]];
    let b = arr2(&[[6., 5., 4.],
                   [3., 2., 1.]]);
    println!("a={}", a);
    println!("a:ndim()={} len()={} shape={:?}", a.ndim(), a.len(), a.shape());
    println!("b={}", b);
    println!("a+b={}", &a+&b);
    println!("a-b={}", &a-&b);
    println!("a*b={}", &a*&b);
    println!("a/b={}", &a/&b);
    println!("a.dot(b.t())={}", &a.dot(&b.t()));
    println!("a.sum()={}", &a.sum());
    println!("a.flatten()={}", Array::from_iter(a.iter()));

    let c = Array::range(0., 6., 1.).into_shape((3,2)).unwrap();
    println!("c={}", &c);
}
