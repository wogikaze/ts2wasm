let arr = [0, 1, 0, 2, 0, 3];
let filtered = arr.filter(x => x);
if (filtered.length === 3) { console.log(1); } else { console.log(0); }
if (filtered[0] === 1) { console.log(1); } else { console.log(0); }
if (filtered[1] === 2) { console.log(1); } else { console.log(0); }
if (filtered[2] === 3) { console.log(1); } else { console.log(0); }
