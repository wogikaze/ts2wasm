const buffer = new ArrayBuffer(16);
const base = new DataView(buffer);
const view = new DataView(buffer, 4);

view.setUint16(0, 0x1234);
view.setUint16(2, 0x1234, true);
view.setInt32(4, 0x01020304);
view.setInt32(8, 0x01020304, true);

console.log(base.getUint8(4));
console.log(base.getUint8(5));
console.log(base.getUint8(6));
console.log(base.getUint8(7));
console.log(base.getInt32(8));
console.log(base.getInt32(12, true));
console.log(view.getUint16(0));
console.log(view.getUint16(2, true));
