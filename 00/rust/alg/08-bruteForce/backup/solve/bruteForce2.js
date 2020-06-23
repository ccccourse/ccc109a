function f(x) {
  return Math.sin(x*x+2*x)/x*x*x;
}

for (var x=-100; x<=100; x+=0.001) {
  if (Math.abs(f(x)) < 0.001)
    console.log("x=", x, " f(x)=", f(x));
}