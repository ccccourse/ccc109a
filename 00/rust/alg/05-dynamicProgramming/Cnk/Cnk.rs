// http://mathworld.wolfram.com/PascalsFormula.html
// https://en.wikipedia.org/wiki/Pascal%27s_rule
// https://en.wikipedia.org/wiki/Pascal%27s_triangle
// https://en.wikipedia.org/wiki/Binomial_coefficient
/*
c(n, k) = 0                        , if n < k
        = 1                        , if k = 0 or k = n
        = c(n-1, k-1) + c(n-1, k)  , if k <= n-k
        = c(n, n-k)                , if k > n-k
*/

fn cnk(n:usize, k:usize)->usize {
  let mut craw = vec![0; (n+1) * (n+1)];
  let mut cbase: Vec<_> = craw.as_mut_slice().chunks_mut(n+1).collect();
  let c: &mut [&mut [_]] = cbase.as_mut_slice();
  for ni in 0..n+1 {
    c[ni][ni] = 1;
    c[ni][0] = 1;
  }
  for ni in 1..n+1 {
    for ki in 1..ni {
        c[ni][ki] = c[ni-1][ki-1] + c[ni-1][ki]
    }
  }
  return c[n][k];
}

fn main() {
  println!("c(5,2)={}", cnk(5,2));
  println!("c(7,3)={}", cnk(7,3));
  println!("c(12,5)={}", cnk(12,5));
  println!("c(60,30)={}", cnk(60,30));
}
