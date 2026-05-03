// Array.prototype.findLast basic tests with ArrowFn callback
const arr = [1, 2, 3, 4, 5];
// findLast: should find element 4 (last element > 3)
const lastBig = arr.findLast(x => x > 3);
console.log(lastBig);

// findLast: should be undefined (no element > 10)
const none = arr.findLast(x => x > 10);
console.log(none);

// findLastIndex: should return 3 (index of element 4)
const lastBigIdx = arr.findLastIndex(x => x > 3);
console.log(lastBigIdx);

// findLastIndex: should return -1
const noneIdx = arr.findLastIndex(x => x > 10);
console.log(noneIdx);
