let state = { calls: 0 };
let mapped = [1, , 3].map(function (value) {
  this.calls = this.calls + 1;
  return value * 2;
}, state);

console.log(state.calls);
console.log(0 in mapped);
console.log(1 in mapped);
console.log(2 in mapped);
console.log(mapped[0]);
console.log(mapped[1]);
console.log(mapped[2]);
console.log(mapped.length);
