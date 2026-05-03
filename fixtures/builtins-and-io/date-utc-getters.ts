let epoch = new Date(0);
console.log(`epoch getTime: ${epoch.getTime()}`);
console.log(`getUTCMilliseconds: ${epoch.getUTCMilliseconds()}`);
console.log(`getUTCSeconds: ${epoch.getUTCSeconds()}`);
console.log(`getUTCMinutes: ${epoch.getUTCMinutes()}`);
console.log(`getUTCHours: ${epoch.getUTCHours()}`);
console.log(`getUTCDay: ${epoch.getUTCDay()}`);
console.log(`getUTCDate: ${epoch.getUTCDate()}`);
console.log(`getUTCMonth: ${epoch.getUTCMonth()}`);
console.log(`getUTCFullYear: ${epoch.getUTCFullYear()}`);

let later = new Date(3661000);
console.log(`later getTime: ${later.getTime()}`);
console.log(`later getUTCSeconds: ${later.getUTCSeconds()}`);
console.log(`later getUTCMinutes: ${later.getUTCMinutes()}`);
console.log(`later getUTCHours: ${later.getUTCHours()}`);

let yesterday = new Date(-86400000);
console.log(`yesterday getTime: ${yesterday.getTime()}`);
console.log(`yesterday getUTCDate: ${yesterday.getUTCDate()}`);
console.log(`yesterday getUTCMonth: ${yesterday.getUTCMonth()}`);
console.log(`yesterday getUTCFullYear: ${yesterday.getUTCFullYear()}`);
console.log(`yesterday getUTCDay: ${yesterday.getUTCDay()}`);
