const buf = new ArrayBuffer(20);
const view = new DataView(buf);

view.setInt32(0, 0x01020304);
view.setInt32(4, 0x01020304, true);
view.setUint32(8, 0x05060708);
view.setUint32(12, 0x05060708, true);
view.setInt32(16, -2, true);

console.log(view.getUint8(0));
console.log(view.getUint8(1));
console.log(view.getUint8(2));
console.log(view.getUint8(3));
console.log(view.getUint8(4));
console.log(view.getUint8(5));
console.log(view.getUint8(6));
console.log(view.getUint8(7));
console.log(view.getUint32(8));
console.log(view.getUint32(12, true));
console.log(view.getInt32(16, true));
