// Array methods: join, includes, indexOf, concat, reverse, slice
// Exercises: common Array prototype methods

// join
let arr = [1, 2, 3];
console.log(arr.join());
console.log(arr.join(","));
console.log(arr.join("-"));

// includes
console.log(arr.includes(2));
console.log(arr.includes(5));

// indexOf
console.log(arr.indexOf(2));
console.log(arr.indexOf(5));

// concat
let b = [4, 5];
let c = arr.concat(b);
console.log(c.length);
console.log(c[0]);
console.log(c[3]);

// reverse
arr.reverse();
console.log(arr[0]);
console.log(arr[1]);
console.log(arr[2]);

// slice
let sliced = arr.slice(0, 2);
console.log(sliced.length);
console.log(sliced[0]);
console.log(sliced[1]);
