const buf = new ArrayBuffer(8);
const view = new DataView(buf);

view.setFloat32(0, 6.25, true);

console.log(view.getFloat32(0, true));
