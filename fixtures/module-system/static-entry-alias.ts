// Fixture for issue 233 static named import aliases backed by a literal export.
import { value as renamed } from "./static-entry-source";
console.log(renamed);
