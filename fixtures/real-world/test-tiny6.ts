const v = "1.2.3";
const dot1 = v.indexOf(".");
const rest1 = v.slice(dot1);
const dot2 = rest1.indexOf(".");
console.log("rest: " + rest1 + " dot2: " + dot2);
