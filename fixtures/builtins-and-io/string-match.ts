// String.prototype.match with RegExp literal
const str = "hello world 42";
const result1 = str.match(/world/);
if (result1 !== null) { console.log(1); } else { console.log(0); }

const result2 = str.match(/xyz/);
if (result2 === null) { console.log(1); } else { console.log(0); }
