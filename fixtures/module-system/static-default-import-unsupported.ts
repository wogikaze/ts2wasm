// Diagnostic fixture for parsed-but-unsupported combined default/named imports tracked by issue 055.
import value, { named as renamed } from "./module-source";
console.log(value, renamed);
