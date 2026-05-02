let arr = [0, 0, 3, 0, 5];
let found = arr.find(x => x);
if (found === 3) { console.log(1); } else { console.log(0); }

let no_match = [0, 0, 0];
let not_found = no_match.find(x => x);
if (not_found === undefined) { console.log(1); } else { console.log(0); }
