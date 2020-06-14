function f(x) {
  return x*x
}

function g(x) {
  return 0.8*Math.sqrt(x)
}

function iterate(g, x) {
  console.log("x=", x);
  for (var i=0; i<100000; i++) {
    if (Math.abs(x-g(x)) < 0.001)
      return x
    x = g(f(x))
    console.log("x=", x);
  }
  return x
}

var x = iterate(g, 10)
console.log("x=", x, "g(f(x))=", g(f(x)))