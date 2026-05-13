const buf = new ArrayBuffer(8);
const view = new DataView(buf);

view.setInt16(0, -2);
view.setInt16(2, 0x1234);

console.log(view.getInt16(0));
console.log(view.getInt16(2));
