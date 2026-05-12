// TypedArray constructors for all concrete typed array families.
const int8 = new Int8Array(1);
int8[0] = -1;
console.log(int8.length);

const uint8 = new Uint8Array([1, 2]);
console.log(uint8.length);

const uint8Clamped = new Uint8ClampedArray(uint8);
console.log(uint8Clamped[1]);

const int16 = new Int16Array(1);
int16[0] = -2;
console.log(int16[0]);

const uint16 = new Uint16Array([3]);
console.log(uint16[0]);

const int32 = new Int32Array(1);
int32[0] = -4;
console.log(int32[0]);

const uint32 = new Uint32Array([5]);
console.log(uint32[0]);

const float32 = new Float32Array(1);
float32[0] = 6;
console.log(float32[0]);

const float64 = new Float64Array([7]);
console.log(float64[0]);

const bigInt64 = new BigInt64Array(1);
bigInt64[0] = 8n;
console.log(bigInt64.length);

const bigUint64 = new BigUint64Array(1);
bigUint64[0] = 9n;
console.log(bigUint64.length);
