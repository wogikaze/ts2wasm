let sparse = [1, , 3];
let arr = [...sparse];

console.log(arr.length);
console.log(0 in arr);
console.log(1 in arr);
console.log(2 in arr);
console.log(arr[0]);
console.log(arr[1] === undefined);
console.log(arr[2]);
