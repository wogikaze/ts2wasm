// Relational comparisons
console.log(1 < 2);
console.log(2 < 1);
console.log(1 <= 2);
console.log(2 <= 2);
console.log(3 <= 2);
console.log(3 > 2);
console.log(2 > 3);
console.log(3 >= 2);
console.log(2 >= 2);
console.log(1 >= 2);

// Strict equality
console.log(1 === 1);
console.log(1 === 2);
console.log(1 === "1");
console.log(true === 1);
console.log(null === null);
console.log(undefined === undefined);
console.log(null === undefined);
console.log(0 === false);
console.log("" === false);

// Strict inequality
console.log(1 !== 1);
console.log(1 !== 2);
console.log(1 !== "1");

// Loose equality (with coercion)
console.log(1 == 1);
console.log(1 == "1");
console.log(0 == false);
console.log("" == false);
console.log(null == undefined);
console.log(null == null);
console.log(undefined == undefined);
console.log(0 == "");
console.log(1 == true);
console.log("1" == true);

// Loose inequality
console.log(1 != 2);
console.log(1 != "1");
console.log(0 != false);
