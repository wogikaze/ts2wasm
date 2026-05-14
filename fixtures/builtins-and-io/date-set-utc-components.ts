let date = new Date(0);

console.log(date.setUTCMonth(1, 3));
console.log(date.getUTCFullYear());
console.log(date.getUTCMonth());
console.log(date.getUTCDate());

console.log(date.setUTCDate(4));
console.log(date.getUTCDate());

console.log(date.setUTCHours(5, 6, 7, 8));
console.log(date.getUTCHours());
console.log(date.getUTCMinutes());
console.log(date.getUTCSeconds());
console.log(date.getUTCMilliseconds());

console.log(date.setUTCMinutes(9, 10, 11));
console.log(date.getUTCMinutes());
console.log(date.getUTCSeconds());
console.log(date.getUTCMilliseconds());

console.log(date.setUTCSeconds(12, 13));
console.log(date.getUTCSeconds());
console.log(date.getUTCMilliseconds());

console.log(date.setUTCMilliseconds(14));
console.log(date.getUTCMilliseconds());
