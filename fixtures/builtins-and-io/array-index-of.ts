let arr = [1, 2, 3, 2, 1];
let idx = arr.indexOf(2);
if (idx === 1) { console.log(1); } else { console.log(0); }

let missing = arr.indexOf(99);
if (missing === -1) { console.log(1); } else { console.log(0); }

let first = arr.indexOf(1);
if (first === 0) { console.log(1); } else { console.log(0); }
