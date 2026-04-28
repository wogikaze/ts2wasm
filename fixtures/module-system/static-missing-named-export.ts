// Diagnostic fixture for issue 233 unresolved named exports from an existing local module.
import { missing } from "./static-entry-source";
console.log(missing);
