let a = 123456789012345678901234567890n;
let b = 12345678901234567890n;
console.log(a / b);
console.log(a % b);

let neg = -123456789012345678901234567890n;
console.log(neg / b);
console.log(neg % b);
console.log(a / -12345678901234567890n);
console.log(a % -12345678901234567890n);

let justOutsideSigned = 9223372036854775808n;
let three = 3n;
console.log(justOutsideSigned / three);
console.log(justOutsideSigned % three);
