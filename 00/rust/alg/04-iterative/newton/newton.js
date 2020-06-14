/*
牛頓逼近法求叠代式及應用 -- https://www.itread01.com/articles/1490047226.html

切線方程式 y  = f(x0) + f'(x0)(x-x0)

y = 0 時 0 = f(x0) + f'(x0)(x-x0)

=> -f(x0) = f'(x0)(x-x0)

=> -f(x0)/f'(x0) = x-x0

=> x0-f(x0)/f'(x0) = x

*/

let dx = 0.000001
let dmin = 0.000000001

function f(x) {
  // return x*x - 3
  return x*x*x - 8
}

function df(f, x) {
  return (f(x+dx)-f(x))/dx
}

function nextx(f, x) {
  return x-f(x)/df(f, x)
}

function iterate(x) {
  var xLast = x
  while (1) {
    console.log("x=", x)
    x = nextx(f, x)
    if (Math.abs(xLast-x) < dmin) break
    xLast = x
  }
  return x
}

console.log('sqrt(3) = ', iterate(1))
