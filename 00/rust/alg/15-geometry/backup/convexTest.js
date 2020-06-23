let G = require('./geometry')

var points = [[0,0],[1,0],[1,1],[0,1],[.5,.5],[-1,-1]];
var convex = G.convexHall(points)
console.log(convex)