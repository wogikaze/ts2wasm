const buf = new ArrayBuffer(16);
const view = new DataView(buf);

view.setFloat64(4, 3.14, true);

console.log(view.getFloat64(4, true));
