const buf = new ArrayBuffer(8);
const view = new DataView(buf);

view.setInt8(0, -1);
view.setInt8(1, -128);
view.setUint8(2, 255);
view.setUint8(3, 128);

console.log(view.getInt8(0));
console.log(view.getInt8(1));
console.log(view.getUint8(2));
console.log(view.getUint8(3));
