const buf = new ArrayBuffer(32);
const base = new DataView(buf);
const view = new DataView(buf, 4);

view.setFloat32(4, 6.25, true);
view.setFloat64(8, 3.14, true);

console.log(base.getFloat32(8, true));
console.log(base.getFloat64(12, true));
