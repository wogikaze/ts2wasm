let callCnt = 0;

function callbackfn(value) {
  callCnt++;
  return 1;
}

let srcArr = new Array(10);
srcArr[1] = undefined;
let resArr = srcArr.map(callbackfn);

console.log(resArr.length);
console.log(callCnt);
console.log(0 in resArr);
console.log(1 in resArr);
console.log(2 in resArr);
console.log(resArr[1]);
