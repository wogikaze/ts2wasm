const v = "1.2.3";
const dot1 = v.indexOf(".");
const n = dot1 + 1;
const rest1 = v.slice(n);
const dot2 = rest1.indexOf(".");
const minorS = rest1.slice(0, dot2);
console.log("minor: " + minorS);
