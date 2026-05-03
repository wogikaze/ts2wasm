// Array.prototype.shift - removes and returns first element
let arr: number[] = [1, 2, 3];
let first = arr.shift();
if (arr[0] === 2) { console.log(1); } else { console.log(0); }
if (arr.length === 2) { console.log(1); } else { console.log(0); }

// Array.prototype.unshift - adds element at beginning, returns new length
let arr2: number[] = [2, 3];
let newLen = arr2.unshift(1);
if (arr2[0] === 1) { console.log(1); } else { console.log(0); }
if (newLen === 3) { console.log(1); } else { console.log(0); }

// Array.prototype.splice - removes elements, returns removed array
let arr3: number[] = [1, 2, 3, 4];
let removed = arr3.splice(1, 2);
if (removed[0] === 2) { console.log(1); } else { console.log(0); }
if (removed[1] === 3) { console.log(1); } else { console.log(0); }
if (arr3.length === 2) { console.log(1); } else { console.log(0); }
if (arr3[0] === 1) { console.log(1); } else { console.log(0); }
if (arr3[1] === 4) { console.log(1); } else { console.log(0); }
