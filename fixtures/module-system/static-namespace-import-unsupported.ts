// Diagnostic fixture for unsupported static namespace imports tracked by issue 055.
import * as mod from "./module-source";
console.log(mod.value);
