// Test that ArrayBuffer, DataView, and all TypedArray constructors
// are recognized as known global names without requiring runtime implementation.
const ab: any = ArrayBuffer;
const dv: any = DataView;
const i8: any = Int8Array;
const ui8: any = Uint8Array;
const ui8c: any = Uint8ClampedArray;
const i16: any = Int16Array;
const ui16: any = Uint16Array;
const i32: any = Int32Array;
const ui32: any = Uint32Array;
const f32: any = Float32Array;
const f64: any = Float64Array;
const bi64: any = BigInt64Array;
const bui64: any = BigUint64Array;
console.log(ab, dv, i8, ui8, ui8c, i16, ui16, i32, ui32, f32, f64, bi64, bui64);
