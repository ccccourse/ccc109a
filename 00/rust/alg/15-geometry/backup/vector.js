const V = module.exports = {}

// 向量相加 : a+b = [a0+b0, a1+b1, ...]
V.add = function (a,b) {
  let len = a.length, r = new Array(len)
  for (let i=0; i<len; i++) {
    r[i] = a[i] + b[i]
  }
  return r
}

// 向量相減 : a-b = [a0-b0, a1-b1, ...]
V.sub = function (a,b) {
  let len = a.length, r = new Array(len)
  for (let i=0; i<len; i++) {
    r[i] = a[i] - b[i]
  }
  return r
}

// 內積 a0*b0+a1*b1+...
V.dot = function (a,b) {
  let len = a.length, r = 0
  for (let i=0; i<len; i++) {
    r += a[i] * b[i]
  }
  return r
}

// 叉積 a0*b1-a1*b0 (只適用於二維向量)
V.cross = function (a,b) {
  return a[0]*b[1]-a[1]*b[0]
}

// 向量的長度
V.length = function (a) {
  let len = a.length, r = 0
  for (let i=0; i<len; i++) {
    r += a[i] * a[i]
  }
  return r
}

// 向量的長度
V.length = function (a) {
  let len = a.length, r = 0
  for (let i=0; i<len; i++) {
    r += a[i] * a[i]
  }
  return Math.sqrt(r)
}

// 將向量正規化 (長度 = 1)
V.normalize = function (a) {
  let alen = V.length(a), len = a.length, r = new Array(len)
  for (let i=0; i<len; i++) {
    r[i] = a[i]/alen
  }
  return r
}

// 偽極角
V.pseudoPolarAngle = function (v, v0) {
  let vd = V.sub(v, v0)
  let [x, y] = V.normalize(vd)
  // console.log('x,y=', x, y)
  if (x>=0 && y>=0) return y
  if (x<0 && y >=0) return (Math.PI/2) - x
  if (x<0 && y<0) return Math.PI-y
  return Math.PI*3/2+x
}
