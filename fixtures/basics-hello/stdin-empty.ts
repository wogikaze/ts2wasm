// Stdin edge case: empty piped input should not hang and should produce empty output
const data = require("fs").readFileSync(0, "utf8");
console.log(data);
