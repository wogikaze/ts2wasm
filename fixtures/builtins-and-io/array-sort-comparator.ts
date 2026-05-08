// Array.prototype.sort with comparator (should build)
const arr = [3, 1, 4, 1, 5, 9];
arr.sort((a: number, b: number) => a - b);
console.log(arr[0]);
console.log(arr[1]);
console.log(arr[arr.length - 1]);
