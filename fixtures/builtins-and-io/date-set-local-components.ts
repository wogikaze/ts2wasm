let date = new Date(0);

// setFullYear
console.log(date.setFullYear(2026));
console.log(date.getFullYear());

// setMonth
console.log(date.setMonth(5, 15));
console.log(date.getMonth());
console.log(date.getDate());

// setDate
console.log(date.setDate(20));
console.log(date.getDate());

// setHours (with optional minutes, seconds, ms)
console.log(date.setHours(10, 30, 45, 500));
console.log(date.getHours());
console.log(date.getMinutes());
console.log(date.getSeconds());
console.log(date.getMilliseconds());

// setMinutes (with optional seconds, ms)
console.log(date.setMinutes(15, 20, 100));
console.log(date.getMinutes());
console.log(date.getSeconds());
console.log(date.getMilliseconds());

// setSeconds (with optional ms)
console.log(date.setSeconds(50, 250));
console.log(date.getSeconds());
console.log(date.getMilliseconds());

// setMilliseconds
console.log(date.setMilliseconds(750));
console.log(date.getMilliseconds());
