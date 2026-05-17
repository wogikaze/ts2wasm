// String.split with RegExp separator
const parts1 = "a,b,c".split(/,/);
console.log(parts1.length);
console.log(parts1[0]);
console.log(parts1[1]);
console.log(parts1[2]);

const parts2 = "hello world  foo".split(/\s+/);
console.log(parts2.length);
console.log(parts2[0]);
console.log(parts2[1]);
console.log(parts2[2]);

const parts3 = "test".split(/x/);
console.log(parts3.length);
console.log(parts3[0]);
