// Test: function declaration value binding
function foo() { return 42; }

// Direct call
console.log(foo());

// Optional call on function declaration (should work since it resolves as a function)
let x = foo?.();
console.log(x);

function bar(y: number) { return y + 10; }

// Normal call
console.log(bar(5));

// Multiple function declarations accumulate correctly
function add(a: number, b: number) { return a + b; }
console.log(add(3, 4));
console.log(add(100, 200));
