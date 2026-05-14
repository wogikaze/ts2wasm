// WASI filesystem test: read and write files using Node.js fs API
// This uses require("fs") which is resolved through the module system's
// builtin resolution to RuntimeFn and HostImport based WASI operations.

let text = require("fs").readFileSync("./input.txt", "utf8");
console.log(text);

require("fs").writeFileSync("./output.txt", "wasi-output");
console.log("done");
