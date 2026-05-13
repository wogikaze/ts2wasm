const buf = new ArrayBuffer(12);
const view = new DataView(buf);

view.setInt32(0, -12345);
view.setInt32(4, 65536);

console.log(view.getInt32(0));
console.log(view.getInt32(4));
