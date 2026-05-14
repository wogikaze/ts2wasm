function check(actual: any, expected: any) {
  if (actual === expected) {
    console.log("ok");
  } else {
    console.log("bad:" + actual + ":" + expected);
  }
}

check(Number.isFinite(42), true);
check(Number.isNaN(NaN), true);
check(Number.isInteger(42), true);
check(Number.isSafeInteger(2147483647), true);
check(parseInt("ff", 16), 255);
check(parseInt("101", 2), 5);
check(parseInt("z", 36), 35);
check(parseFloat("  -7"), -7);
check(Number.parseInt("ff", 16), 255);
check(Number.parseInt("101", 2), 5);
check(Number.parseInt("z", 36), 35);
check(Number.parseFloat("  -7"), -7);
check((42).toFixed(), "42");
check((42).toFixed(2), "42.00");
check((42).toExponential(), "4.2e+1");
check((42).toExponential(3), "4.200e+1");
check((42).toPrecision(), "42");
check((42).toPrecision(4), "42.00");

// Number static and prototype methods test
let n: any = 123.456;

// Number.isNaN — use a non-number expression
let nan_val: any = "abc" / 1;
console.log(Number.isNaN(nan_val));
console.log(Number.isNaN(123));

// Number.isFinite
console.log(Number.isFinite(123));
console.log(Number.isFinite(1 / 0));

// Number.isInteger
console.log(Number.isInteger(42));
console.log(Number.isInteger(42.5));

// Number.isSafeInteger
console.log(Number.isSafeInteger(42));

// parseInt/parseFloat aliases
console.log(Number.parseInt("42"));
console.log(Number.parseFloat("3.14"));

// toFixed
console.log(n.toFixed(2));
console.log(n.toFixed(0));

// toExponential
console.log(n.toExponential(2));

// toPrecision
let n2: any = 123.456;
console.log(n2.toPrecision(4));
console.log(n2.toPrecision(2));

// Number static methods: isNaN, isFinite, isInteger, isSafeInteger (incoming)
console.log(Number.isNaN(NaN));
console.log(Number.isNaN(123));
console.log(Number.isFinite(Infinity));
console.log(Number.isFinite(123));
console.log(Number.isInteger(123));
console.log(Number.isInteger(123.456));
console.log(Number.isSafeInteger(123));
console.log(Number.isSafeInteger(123));
