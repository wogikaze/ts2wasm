// Diagnostic fixture for missing local static namespace imports tracked by issue 232.
import * as mod from "./module-source";
console.log(mod.value);
