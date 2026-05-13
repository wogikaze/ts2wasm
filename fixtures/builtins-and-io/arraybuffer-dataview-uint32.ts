const buf = new ArrayBuffer(12);
const view = new DataView(buf);

view.setUint32(0, 65535);
view.setUint32(4, 1024);

console.log(view.getUint32(0));
console.log(view.getUint32(4));
