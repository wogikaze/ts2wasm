// Array.prototype.reduce — basic numeric sum
let arr = [1, 2, 3, 4, 5];
let sum = arr.reduce((acc, v) => acc + v, 0);
if (sum === 15) { console.log(1); } else { console.log(0); }

// Array.prototype.reduce — string concatenation
let words = ["a", "b", "c"];
let concat = words.reduce((acc, v) => acc + v, "");
if (concat === "abc") { console.log(1); } else { console.log(0); }

// Array.prototype.reduce — with index
let indices = [10, 20, 30];
let withIdx = indices.reduce((acc, v, i) => acc + v + i, 0);
if (withIdx === 63) { console.log(1); } else { console.log(0); }

// Array.prototype.reduce — empty array with initial
let empty: number[] = [];
let emptyResult = empty.reduce((acc, v) => acc + v, 99);
if (emptyResult === 99) { console.log(1); } else { console.log(0); }

// Array.prototype.reduceRight — sum in reverse
let rev = [1, 2, 3];
let revSum = rev.reduceRight((acc, v) => acc * 10 + v, 0);
if (revSum === 321) { console.log(1); } else { console.log(0); }

// Array.prototype.reduceRight — string concat in reverse
let revWords = ["a", "b", "c"];
let revConcat = revWords.reduceRight((acc, v) => acc + v, "");
if (revConcat === "cba") { console.log(1); } else { console.log(0); }
