let some_truthy = [0, 0, 3];
let result1 = some_truthy.some(x => x);
if (result1) { console.log(1); } else { console.log(0); }

let all_falsy = [0, 0, 0];
let result2 = all_falsy.some(x => x);
if (result2) { console.log(1); } else { console.log(0); }

let empty = [];
let result3 = empty.some(x => x);
if (result3) { console.log(1); } else { console.log(0); }
