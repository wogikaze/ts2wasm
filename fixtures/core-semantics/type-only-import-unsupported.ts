// type-only import — should produce unsupported diagnostic
import type { SomeType } from "./nonexistent-module";
console.log("after type-only import");
