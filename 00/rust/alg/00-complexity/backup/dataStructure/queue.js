class Queue {
  constructor() {
    this.a = []
  }

  enqueue(o) {
    this.a.push(o)
  }

  dequeue() {
    return this.a.shift()
  }
}


var q = new Queue()
q.enqueue(3)
console.log("q=%j", q)
q.enqueue(5)
console.log("q=%j", q)
q.enqueue(2)
console.log("q=%j", q)
var t1 = q.dequeue()
console.log("t1=%j", t1)
console.log("q=%j", q)
