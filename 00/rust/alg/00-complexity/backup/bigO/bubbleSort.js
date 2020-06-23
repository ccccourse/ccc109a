function sort(a) {
  let len = a.length
  for (let i=0; i<len; i++) {
    for (let j=0; j<i; j++) {
      if (a[j] > a[i]) {
        let t = a[i]
        a[i] = a[j]
        a[j] = t
      }
    }
  }
  return a
}

console.log('sort([3, 8, 2, 1, 5]=', sort([3,8,2,1,5]))
