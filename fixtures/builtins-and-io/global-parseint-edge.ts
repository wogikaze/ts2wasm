// parseInt edge cases: trailing chars, trailing spaces, Number input, globalThis

// Trailing non-digit characters
console.log(parseInt("42abc"));
console.log(parseInt("101xyz"));

// Trailing spaces
console.log(parseInt("  42  "));
console.log(parseInt("  101  "));

// Leading/trailing mixed
console.log(parseInt("  42abc  "));

// globalThis.parseInt
console.log(globalThis.parseInt("42"));
console.log(globalThis.parseInt("0xFF"));
console.log(globalThis.parseInt("101", 2));

// Number.parseInt (alias)
console.log(Number.parseInt("42"));
console.log(Number.parseInt("0xFF"));

// Number input edge cases
console.log(parseInt(42));
console.log(parseInt(-1));
console.log(isNaN(parseInt(Infinity)));
console.log(isNaN(parseInt(NaN)));
console.log(parseInt(-0));
