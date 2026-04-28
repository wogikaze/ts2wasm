// Diagnostic fixture for missing local combined default/named imports tracked by issue 232.
import value, { named as renamed } from "./module-source";
console.log(value, renamed);
