let V = require('./vector')

let x = [1,2], y = [3,4]

console.log('add(x, y)=', V.add(x,y))
console.log('sub(x, y)=', V.sub(x,y))
console.log('dot(x, y)=', V.dot(x,y))
console.log('cross(x, y)=', V.cross(x,y))
console.log('pseudoPolarAngle(x, y)=', V.pseudoPolarAngle(x,y))