let date = new Date(3661000);
console.log(date.getTime());
console.log(date.getUTCFullYear());
console.log(date.getUTCMonth());
console.log(date.getUTCDate());
console.log(date.getUTCHours());
console.log(date.getUTCMinutes());
console.log(date.getUTCSeconds());
console.log(date.getUTCMilliseconds());

// setUTCMonth
console.log("--- setUTCMonth ---");
console.log(date.setUTCMonth(5));
console.log(date.getTime());
console.log(date.getUTCMonth());

// setUTCDate
console.log("--- setUTCDate ---");
console.log(date.setUTCDate(15));
console.log(date.getTime());
console.log(date.getUTCDate());

// setUTCHours
console.log("--- setUTCHours ---");
console.log(date.setUTCHours(10));
console.log(date.getTime());
console.log(date.getUTCHours());

// setUTCMinutes
console.log("--- setUTCMinutes ---");
console.log(date.setUTCMinutes(30));
console.log(date.getTime());
console.log(date.getUTCMinutes());

// setUTCSeconds
console.log("--- setUTCSeconds ---");
console.log(date.setUTCSeconds(45));
console.log(date.getTime());
console.log(date.getUTCSeconds());

// setUTCMilliseconds
console.log("--- setUTCMilliseconds ---");
console.log(date.setUTCMilliseconds(500));
console.log(date.getTime());
console.log(date.getUTCMilliseconds());

// Verify all components
console.log("--- final state ---");
console.log(date.getUTCFullYear());
console.log(date.getUTCMonth());
console.log(date.getUTCDate());
console.log(date.getUTCHours());
console.log(date.getUTCMinutes());
console.log(date.getUTCSeconds());
console.log(date.getUTCMilliseconds());
