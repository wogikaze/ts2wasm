// ArrayBuffer/DataView basic usage
const buf = new ArrayBuffer(16);
const view = new DataView(buf);
view.setInt32(0, 42);
view.setFloat64(4, 3.14);
console.log(view.getInt32(0));
console.log(view.getFloat64(4));
