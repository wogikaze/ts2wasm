const buf = new ArrayBuffer(8);
const view = new DataView(buf);

view.setUint16(0, 65535);
view.setUint16(2, 0x1234);

console.log(view.getUint16(0));
console.log(view.getUint16(2));
