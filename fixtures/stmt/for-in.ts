let obj: any = { a: 1, b: 2 };
let s = 0;
for (let k in obj) { s += obj[k]; }
console.log(s);
