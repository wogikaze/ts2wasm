let has = "hello world".includes("world");
if (has) { console.log(1); } else { console.log(0); }

let nope = "hello world".includes("xyz");
if (nope) { console.log(1); } else { console.log(0); }

let empty = "hello".includes("");
if (empty) { console.log(1); } else { console.log(0); }
