// new String() constructor tests
// Test 1: new String with string arg
let s1: any = new String("hello");
console.log(s1);

// Test 2: new String with number arg
let s2: any = new String(42);
console.log(s2);

// Test 3: new String with no args
let s3: any = new String();
console.log(s3);

// Test 4: new String with undefined
let s4: any = new String(undefined);
console.log(s4);
