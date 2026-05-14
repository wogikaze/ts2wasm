// Comprehensive Date methods test
// Note: host-import methods (toString, toISOString, toJSON, toDateString, toTimeString, getTimezoneOffset, local getters)
// are tested in build_smoke only, not node_diff, because iwasm doesn't have Date host shims
let epoch = new Date(0);
let later = new Date(3661000);
let yesterday = new Date(-86400000);

// Constructor
console.log(epoch.getTime());
console.log(later.getTime());
console.log(yesterday.getTime());

// getTime / valueOf
console.log(epoch.getTime());
console.log(epoch.valueOf());
console.log(later.valueOf());

// UTC getters (pure WAT math, no host imports needed)
console.log(epoch.getUTCMilliseconds());
console.log(epoch.getUTCSeconds());
console.log(epoch.getUTCMinutes());
console.log(epoch.getUTCHours());
console.log(epoch.getUTCDay());
console.log(epoch.getUTCDate());
console.log(epoch.getUTCMonth());
console.log(epoch.getUTCFullYear());
