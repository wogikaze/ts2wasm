let a = "hello".padEnd(10, "x");
if (a === "helloxxxxx") { console.log(1); } else { console.log(0); }

let b = "hello".padEnd(3, "x");
if (b === "hello") { console.log(1); } else { console.log(0); }

let c = "hello".padEnd(8, "abc");
if (c === "helloabc") { console.log(1); } else { console.log(0); }
