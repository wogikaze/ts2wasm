// Array.prototype.filter — basic semantics with arrow callback
let arr = [1, 2, 3, 4, 5];

// filter with simple predicate
let evens = arr.filter(x => x % 2 === 0);
if (evens.length === 2) { console.log(1); } else { console.log(0); }
if (evens[0] === 2) { console.log(1); } else { console.log(0); }
if (evens[1] === 4) { console.log(1); } else { console.log(0); }

// filter returns empty array when no match
let big = arr.filter(x => x > 10);
if (big.length === 0) { console.log(1); } else { console.log(0); }

// filter on empty array
let empty: number[] = [];
let result = empty.filter(x => x > 0);
if (result.length === 0) { console.log(1); } else { console.log(0); }
