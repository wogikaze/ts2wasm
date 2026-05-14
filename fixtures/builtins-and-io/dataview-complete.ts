// DataView complete: all get/set methods
const buf = new ArrayBuffer(32);
const view = new DataView(buf);

// Int8/Uint8
view.setInt8(0, -1);
view.setInt8(1, -128);
view.setUint8(2, 255);
view.setUint8(3, 128);
console.log(view.getInt8(0));
console.log(view.getInt8(1));
console.log(view.getUint8(2));
console.log(view.getUint8(3));

// Int16/Uint16 (little-endian)
view.setInt16(4, -32768, true);
view.setUint16(6, 65535, true);
console.log(view.getInt16(4, true));
console.log(view.getUint16(6, true));

// Int16/Uint16 (big-endian)
view.setInt16(8, -32768, false);
view.setUint16(10, 65535, false);
console.log(view.getInt16(8, false));
console.log(view.getUint16(10, false));

// Int32/Uint32 (little-endian)
view.setInt32(12, -1000000, true);
view.setUint32(16, 3000000, true);
console.log(view.getInt32(12, true));
console.log(view.getUint32(16, true));

// Int32/Uint32 (big-endian)
view.setInt32(20, -1000000, false);
view.setUint32(24, 3000000, false);
console.log(view.getInt32(20, false));
console.log(view.getUint32(24, false));

// Float32/Float64
view.setFloat32(0, 3.14, true);
view.setFloat64(4, 3.141592653589793, true);
console.log(view.getFloat32(0, true));
console.log(view.getFloat64(4, true));

// Float32/Float64 big-endian
view.setFloat32(12, 3.14, false);
view.setFloat64(16, 3.141592653589793, false);
console.log(view.getFloat32(12, false));
console.log(view.getFloat64(16, false));

// buffer, byteOffset, byteLength
console.log(view.buffer === buf);
console.log(view.byteOffset);
console.log(view.byteLength);
