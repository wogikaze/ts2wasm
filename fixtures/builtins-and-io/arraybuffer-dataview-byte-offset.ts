const buf = new ArrayBuffer(12);
const base = new DataView(buf);
const view = new DataView(buf, 4);

view.setInt16(0, -321);
view.setUint8(2, 255);

console.log(base.getInt16(4));
console.log(base.getUint8(6));
console.log(view.getInt16(0));
console.log(view.getUint8(2));
