function factor(n) {
  for (let i=2; i < Math.sqrt(n); i++) {
    if (n % i == 0) return i
  }
  return -1
}

console.log('factor(15)=', factor(15))
console.log('factor(37)=', factor(37))
console.log('factor(9373467139)=', factor(9373467139))
// 以下大質數參考 《維基百科: 質數列表》 -- https://zh.wikipedia.org/wiki/%E8%B3%AA%E6%95%B8%E5%88%97%E8%A1%A8
console.log('factor(10000819)=', factor(10000819))
console.log('factor(3093215881333057)=', factor(3093215881333057))
console.log('factor(489133282872437279)=', factor(489133282872437279)) // 這個超越了 JavaScript 的安全整數表達限制範圍！
console.log('489133282872437279=', 489133282872437279)
console.log('489133282872437279.000001=', 489133282872437279.000001)