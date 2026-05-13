const buf = new ArrayBuffer(12);
const view = new DataView(buf);

view.setInt16(0, 0x1234);
view.setInt16(2, 0x1234, true);
view.setUint16(4, 0x4567);
view.setUint16(6, 0x4567, true);
view.setInt16(8, -2);
view.setInt16(10, -2, true);

console.log(view.getUint8(0));
console.log(view.getUint8(1));
console.log(view.getUint8(2));
console.log(view.getUint8(3));
console.log(view.getUint16(4));
console.log(view.getUint16(6, true));
console.log(view.getInt16(8));
console.log(view.getInt16(10, true));
