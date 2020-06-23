const sha256 = require('crypto-js/sha256')
const base64 = require('crypto-js/enc-base64')

let record = {
  nonce: 0,
  data: 'john => mary $2.7',
}

let json = JSON.stringify(record, null, 2)
console.log('json=', json)
const digest = sha256(json)

console.log('digest=', base64.stringify(digest))
