// Array.prototype.find — basic semantics with arrow callback
let arr = [1, 2, 3, 4, 5];

// find with simple predicate
let found = arr.find(x => x > 3);
if (found === 4) { console.log(1); } else { console.log(0); }

// find returns first match
let first = arr.find(x => x > 1);
if (first === 2) { console.log(1); } else { console.log(0); }

// find returns undefined when no match
let none = arr.find(x => x > 10);
if (none === undefined) { console.log(1); } else { console.log(0); }

// find on empty array
let empty: number[] = [];
let result = empty.find(x => x > 0);
if (result === undefined) { console.log(1); } else { console.log(0); }
