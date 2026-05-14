const buf = new SharedArrayBuffer(4);
const view = new DataView(buf);

view.setUint16(0, 258);
view.setUint16(2, 513);

console.log(view.getUint16(0));
console.log(view.getUint16(2));
