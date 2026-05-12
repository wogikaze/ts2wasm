// Test Date constructor with various epoch millisecond arguments
console.log(new Date(0).getTime());
console.log(new Date(1).getTime());
console.log(new Date(-1).getTime());
console.log(new Date(86400000).getTime());
console.log(new Date(-86400000).getTime());
console.log(new Date(1234567890).getTime());
console.log(new Date(-1234567890).getTime());
