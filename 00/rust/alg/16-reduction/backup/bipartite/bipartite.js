var findMatching = require("bipartite-matching")

console.log(findMatching(5, 5, [
      [0, 0],
      [0, 1],
      [1, 0],
      [2, 1],
      [2, 2],
      [3, 2],
      [3, 3],
      [3, 4],
      [4, 4]
    ]))