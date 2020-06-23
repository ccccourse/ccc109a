// 這個一開始就到答案了，不好的範例。

// 求解 x^2 + y^2 = 4
// 一開始設定 x=1, y=1
let dmin = 0.000001

function getx(y) {
  return Math.sqrt(4-y*y)
}

function gety(x) {
  return Math.sqrt(4-x*x)
}

function iterate(x=1, y=1) {
  var xLast = x, yLast = y
  while (1) {
    console.log("x=", x, 'y=', y)
    x = getx(y)
    y = gety(x)
    if (Math.abs(xLast-x) < dmin) break
    xLast = x
    yLast = y
  }
  return {x, y}
}

var {x,y} = iterate()
console.log("solution: x=", x, 'y=', y)