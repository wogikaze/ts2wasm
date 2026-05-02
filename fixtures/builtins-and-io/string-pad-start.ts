let a = "hello".padStart(10, "x");
if (a === "xxxxxhello") { console.log(1); } else { console.log(0); }

let b = "hello".padStart(3, "x");
if (b === "hello") { console.log(1); } else { console.log(0); }

let c = "hello".padStart(8, "abc");
if (c === "abchello") { console.log(1); } else { console.log(0); }
