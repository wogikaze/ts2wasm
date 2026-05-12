// String to number coercion (unary +)
// Note: float-string parsing is not yet supported (e.g. +"3.14").
console.log(+"42");
console.log(+"0");
console.log(+"");
console.log(+"  ");
console.log(+"0x1A");
console.log(+true);
console.log(+false);
console.log(+null);

// Number to string (string concatenation)
let n = 42;
console.log(n + "");
console.log("The answer is: " + n);

// Truthiness of numeric values
console.log(0 ? "truthy" : "falsy");
console.log(1 ? "truthy" : "falsy");
console.log(-1 ? "truthy" : "falsy");

// Boolean coercion (!)
console.log(!0);
console.log(!1);
console.log(!!0);
console.log(!!42);

// == coercion with different types
console.log("42" == 42);
console.log(0 == "");
console.log("" == false);
console.log(null == undefined);
console.log(null == null);
console.log(undefined == undefined);

// === does not coerce
console.log("42" === 42);
console.log(0 === "");
console.log("" === false);
console.log(null === undefined);
