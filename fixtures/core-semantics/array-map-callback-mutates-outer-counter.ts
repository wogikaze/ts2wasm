let callCnt = 0;

function callbackfn(value) {
  callCnt++;
  return value + 1;
}

let mapped = [1, 2, 3].map(callbackfn);

console.log(callCnt);
console.log(mapped.length);
console.log(mapped[0]);
console.log(mapped[1]);
console.log(mapped[2]);
