let values = [1, -2, 0];
let strings = values.map(n => String(n));
let numbers = strings.map(n => +n);

console.log(numbers.length);
console.log(numbers[0] + 1);
console.log(numbers[1] + 5);
console.log(numbers[2] === 0);
