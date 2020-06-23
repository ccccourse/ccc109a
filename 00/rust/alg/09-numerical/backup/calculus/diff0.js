var diff = function (f, x, dx = 0.000000001) {
  return (f(x+dx)-f(x))/dx
  // var dy = f(x + dx) - f(x)
  // return dy / dx
}

console.log('diff(sin(x), pi/3) = ', diff(Math.sin, Math.PI / 3))
console.log('cos(pi/3) = ', Math.cos(Math.PI / 3))
