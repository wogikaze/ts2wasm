// Dynamic calls not supported by current HIR lowering
// Computed property calls (obj[key]()) are not identifier-based
// and cannot be lowered through the HIR call path.

let obj = { value: 42 };
let key: string = "value";

// This computed property call should be rejected:
// only identifier calls are supported in expression context
console.log(obj[key]());
