const SEED_MAX = 9999997
var d = new Date();
var seed = d.getTime()%SEED_MAX; //371

function random() {
    seed = (seed+37 ) % SEED_MAX
    var x = Math.sin(seed) * 93177
    return x - Math.floor(x);
}

module.exports = random