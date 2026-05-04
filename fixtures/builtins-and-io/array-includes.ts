let arr = [1, 2, 3];
let has = arr.includes(2);
if (has) { console.log(1); } else { console.log(0); }

let nope = arr.includes(99);
if (nope) { console.log(1); } else { console.log(0); }

let empty_arr = [];
let empty = empty_arr.includes(1);
if (empty) { console.log(1); } else { console.log(0); }
