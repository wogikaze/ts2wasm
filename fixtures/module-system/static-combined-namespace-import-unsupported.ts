// Diagnostic fixture for parsed-but-unsupported combined default/namespace imports tracked by issue 055.
import value, * as mod from "./module-source";
console.log(value, mod.value);
