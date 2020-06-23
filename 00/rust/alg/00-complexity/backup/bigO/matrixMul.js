function matrixMul(a, b) {
  let m = a.length, n = a[0].length, p = b[0].length
  let r = new Array(m)
  for (let i=0; i<m; i++) {
    r[i] = new Array(p)
    r[i].fill(0)
  }
  for (let i=0; i<m; i++) {
    for (let j=0; j<n; j++) {
      for (let k=0; k<p; k++) {
        r[i][k] += a[i][j] * b[j][k]
      }
    }
  }
  return r
}

let a = [[1,2,3],[3,2,1]], b=[[1,1],[1,1],[1,1]]
console.log(matrixMul(a,b))
