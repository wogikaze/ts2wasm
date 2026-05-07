// Issue 5136: RegExp/String prototype methods accept 0 args
// RegExp.prototype.test() with 0 args should return false (matches undefined)
const re = /a/;
console.log(re.test());
