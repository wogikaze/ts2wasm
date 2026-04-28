// Diagnostic fixture for unsupported static side-effect imports tracked by issue 055.
import "./module-source";
console.log("loaded");
