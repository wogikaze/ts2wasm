// Fixture for issue 233 once-only static module initialization with repeated imports.
import { value as first } from "./static-entry-source";
import { value as second } from "./static-entry-source";
console.log(first + second);
