// Type-only import syntax
import type { SomeInterface } from "./non-existent-module";

// Dummy usage to prevent dead-code elimination
const x: SomeInterface = { value: 42 };
console.log(x.value);
