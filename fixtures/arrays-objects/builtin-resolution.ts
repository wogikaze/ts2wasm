// Characterization test fixture: builtin resolver domain coverage
// Exercise Math, Object, String, Number builtin resolution
// Used by P9 builtin_resolver domain split acceptance

// Math builtins
const abs: number = Math.abs(-5);
const ceil: number = Math.ceil(3.14);
const floor: number = Math.floor(3.99);
const round: number = Math.round(3.5);
const max: number = Math.max(1, 2, 3);
const min: number = Math.min(1, 2, 3);
const pow: number = Math.pow(2, 3);
const sqrt: number = Math.sqrt(16);

// Object builtins
const keys: string[] = Object.keys({a: 1, b: 2});
const vals: any[] = Object.values({a: 1, b: 2});
const entries: [string, any][] = Object.entries({a: 1});

// String builtins
const upper: string = "hello".toUpperCase();
const lower: string = "HELLO".toLowerCase();
const trimmed: string = "  hi  ".trim();
const sliced: string = "hello".slice(1, 3);
const idx: number = "hello".indexOf("l");
const included: boolean = "hello".includes("ell");
const replaced: string = "hello".replace("l", "x");

// Number builtins
const fixed: string = (3.14159).toFixed(2);
const numStr: string = (42).toString();
