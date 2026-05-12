// TypedArray basic usage: length, array input, typed-array input, and index read/write
const u8 = new Uint8Array(3);
u8[1] = 9;
console.log(u8.length);
console.log(u8[0]);
console.log(u8[1]);
console.log(u8[2]);

const u8copy = new Uint8Array(u8);
console.log(u8copy.length);
console.log(u8copy[1]);

const i32 = new Int32Array([42, -1, 1000]);
i32[1] = -7;
console.log(i32.length);
console.log(i32[0]);
console.log(i32[1]);
