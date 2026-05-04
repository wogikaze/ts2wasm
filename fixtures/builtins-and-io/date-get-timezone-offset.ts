// Date.prototype.getTimezoneOffset tests
let epoch = new Date(0);
let offset = epoch.getTimezoneOffset();
console.log("offset type: " + (typeof offset));
console.log("offset >= -720: " + (offset >= -720));
console.log("offset <= 840: " + (offset <= 840));
