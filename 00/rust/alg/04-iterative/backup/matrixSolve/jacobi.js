/* Jacobi 必需是主對角線絕對優勢的矩陣，否則可能會發散(不收斂)
5x+y  = 6
 x+4y = 5

x = (6-y)/5
y = (5-x)/4

註: 對於 x+2y = 3
        2x+y  = 3

可能會不收斂！
*/

let dmin = 0.000000001


function iterate(x, y) {
  var xLast = x, yLast = y
  while (1) {
    console.log('x=', x, 'y=', y)
    let xNew = (6-y)/5
    let yNew = (5-x)/4
    if (Math.abs(xNew-x) < dmin && Math.abs(yNew-y) < dmin) break
    x = xNew
    y = yNew
  }
  return {x, y}
}

console.log('solve: (x,y) = ', iterate(0, 0))
