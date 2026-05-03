// Array.prototype.findLastIndex basic tests with ArrowFn callback
const arr = [1, 2, 3, 4, 5];
// findLastIndex: should return 3 (index of element 4, last element > 3)
const lastBigIdx = arr.findLastIndex(x => x > 3);
console.log(lastBigIdx);

// findLastIndex: should return -1 (no element > 10)
const noneIdx = arr.findLastIndex(x => x > 10);
console.log(noneIdx);
