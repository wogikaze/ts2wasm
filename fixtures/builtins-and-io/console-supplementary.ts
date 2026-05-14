// Console supplementary edge cases
console.log("log", 1, true, null, undefined);
console.info("info", {a: 1});
console.warn("warn", [1, 2, 3]);
console.debug("debug", "message");

console.group("level1");
console.log("inside level1");
console.group("level2");
console.log("inside level2");
console.groupEnd("level2");
console.log("back to level1");
console.groupEnd("level1");

console.time("t1");
console.timeEnd("t1");

console.count("c1");
console.count("c1");
console.countReset("c1");
console.count("c1");

console.assert(true, "should not print");
console.assert(false, "should print");
console.assert(false);
