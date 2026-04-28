// Diagnostic fixture for missing local combined default/namespace imports tracked by issue 232.
import value, * as mod from "./module-source";
console.log(value, mod.value);
