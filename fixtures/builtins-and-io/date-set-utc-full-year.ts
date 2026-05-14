let date = new Date(3661000);
console.log(date.getTime());
console.log(date.setUTCFullYear(1970, 0, 2));
console.log(date.getTime());
console.log(date.getUTCFullYear());
console.log(date.getUTCMonth());
console.log(date.getUTCDate());
console.log(date.getUTCHours());
console.log(date.getUTCMinutes());
console.log(date.getUTCSeconds());
console.log(date.getUTCMilliseconds());

let sameYear = new Date(86400000);
console.log(sameYear.setUTCFullYear(1970));
console.log(sameYear.getTime());
