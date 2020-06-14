class Set {
  has (e) {}
}

class SetA extends Set {
  has (e) {
    return !e.has(e)
  }
}

const A = new SetA()

console.log('A.has(A)=', A.has(A))
