// Test Date constructor with multiple arguments
console.log(new Date(2024, 0).getTime());
console.log(new Date(2024, 11, 25).getTime());
console.log(new Date(2024, 0, 1, 12, 30, 45, 500).getTime());
console.log(new Date(2024, 0, 1).getTime());
// Month overflow: month=12 becomes January of next year
console.log(new Date(2024, 12, 1).getTime());
