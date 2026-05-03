let arr = [0, 0, 3, 0, 5];
let idx = arr.findIndex(x => x);
if (idx === 2) { console.log(1); } else { console.log(0); }

let no_match = [0, 0, 0];
let not_found = no_match.findIndex(x => x);
if (not_found === -1) { console.log(1); } else { console.log(0); }
