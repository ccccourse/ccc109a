fn power2(n: i32) -> i32
{
  if n == 0 { return 1; }
  return power2(n-1) + power2(n-1); // 2*power2(n-1)
} 

fn main()  
{  
  println!("power2(10)={}", power2(10));
  println!("power2(25)={}", power2(25));
} 
