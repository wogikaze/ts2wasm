// TypedArray byteLength, byteOffset, buffer properties
const a = new Uint8Array([1, 2, 3]);
console.log(a.length);
console.log(a.byteLength);
console.log(a.byteOffset);
console.log(a.buffer);
console.log(Int8Array.BYTES_PER_ELEMENT);
console.log(Uint8Array.BYTES_PER_ELEMENT);
console.log(Int16Array.BYTES_PER_ELEMENT);
console.log(Uint32Array.BYTES_PER_ELEMENT);
console.log(Float64Array.BYTES_PER_ELEMENT);

// ArrayBuffer.byteLength
const buf = new ArrayBuffer(16);
console.log(buf.byteLength);
