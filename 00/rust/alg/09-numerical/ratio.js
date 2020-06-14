var c = console;

class Ratio {
  constructor(a,b) { this.a = a; this.b = b; }
  
  mul(r2) { return new Ratio(this.a*r2.a, this.b*r2.b); }
  
  div(r2) { return new Ratio(this.a*r2.b, this.b*r2.a); }
  
  inv() { return new Ratio(this.b, this.a); }
  
  add(r2) { return new Ratio(this.a*r2.b+this.b*r2.a, this.b*r2.b); }
  
  sub(r2) { return new Ratio(this.a*r2.b-this.b*r2.a, this.b*r2.b); }
  
  toString() { return this.a+'/'+this.b; }

  parse(s) {
    var m = s.match(/^(\d+)(\/(\d+))?$/);
    var a = parseInt(m[1]);
    var b = typeof m[3]==='undefined'?1:parseInt(m[3]);
    return new Ratio(a, b)
  } 
}

Ratio.parse = Ratio.prototype.parse;

var r0 = Ratio.parse('1/2');
c.log(r0);

r0 = Ratio.parse('1');
c.log(r0);

var r1 = new Ratio(2,3);
c.log(r1.toString());

var r2 = r1.mul(r1).add(r1);
c.log(r2.toString());