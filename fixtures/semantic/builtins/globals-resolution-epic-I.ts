// Epic I-20260513: Name Resolution Improvements
// Verify new global names resolve without UnresolvedName errors.

// ES2024/ES2025 globals
let sd = SuppressedError;
let ds = DisposableStack;
let ads = AsyncDisposableStack;
let sr = ShadowRealm;

// test262 host functions
let cr = createRealm;
let dab = detachArrayBuffer;

// Web/HTML API globals
let qm = queueMicrotask;
let sc = structuredClone;
let perf = performance;
let si = setImmediate;
