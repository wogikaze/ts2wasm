// Array.prototype copying methods: with, toReversed, toSorted, toSpliced, findLast, findLastIndex
let arr = [3, 1, 2];

// with
let w = arr.with(1, 10);
console.log(w[0], w[1], w[2]);

// toReversed
let r = arr.toReversed();
console.log(r[0], r[1], r[2]);

// toSorted
let s = arr.toSorted();
console.log(s[0], s[1], s[2]);

// toSpliced
let sp = arr.toSpliced(1, 1);
console.log(sp[0], sp[1], sp.length);

// findLast
let fl = arr.findLast(x => x > 1);
console.log(fl);

// findLastIndex
let fli = arr.findLastIndex(x => x > 1);
console.log(fli);

// original unchanged
console.log(arr[0], arr[1], arr[2]);
