var r = /[\d\-]+/g
var text = `
ccc: 0823-13534 
snoopy: 0977-313456
john: 02-22123456
`

while (true) {
  let m = r.exec(text)
  if (m == null) break
  console.log(m)
}