let s = "hello";
let c1 = s.charCodeAt(0);
if (c1 === 104) { console.log(1); } else { console.log(0); }

let c2 = s.charCodeAt(1);
if (c2 === 101) { console.log(1); } else { console.log(0); }

let c3 = s.charCodeAt(10);
if (isNaN(c3)) { console.log(1); } else { console.log(0); }

let c4 = "".charCodeAt(0);
if (isNaN(c4)) { console.log(1); } else { console.log(0); }
