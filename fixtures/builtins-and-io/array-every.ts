let all_truthy = [1, 2, 3];
let result1 = all_truthy.every(x => x);
if (result1) { console.log(1); } else { console.log(0); }

let has_falsy = [1, 0, 3];
let result2 = has_falsy.every(x => x);
if (result2) { console.log(1); } else { console.log(0); }

let empty = [];
let result3 = empty.every(x => x);
if (result3) { console.log(1); } else { console.log(0); }
