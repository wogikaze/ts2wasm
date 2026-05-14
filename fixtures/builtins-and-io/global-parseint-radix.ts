console.log(parseInt("G", 17));
console.log(parseInt("Z", 36));
console.log(parseInt("z$", 36));
console.log(parseInt("10", 36));
console.log(parseInt("-Z", 36));
console.log(parseInt("1Z", 35));
console.log(isNaN(parseInt("Z", 35)));
console.log(isNaN(parseInt("   ")));
console.log(parseInt("\u20001"));
