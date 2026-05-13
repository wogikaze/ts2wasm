// String static methods and prototype methods for this issue
let c: any = String.fromCharCode(65, 66, 67);
console.log(c);
let c2: any = String.fromCharCode(72, 101, 108, 108, 111);
console.log(c2);
let cp: any = String.fromCodePoint(128169);
console.log(cp);
let cp2: any = String.fromCodePoint(9731, 9733, 9842);
console.log(cp2);
let s: any = "Hello 👋 World";
console.log(s.codePointAt(0));
console.log(s.codePointAt(1));
console.log(s.codePointAt(6));
let s2: any = "abc";
console.log(s2.at(1));
console.log(s2.at(-1));
console.log(s2.at(0));
console.log(s2.at(-3));
