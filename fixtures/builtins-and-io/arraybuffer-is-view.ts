// Test: ArrayBuffer.isView
let buf = new ArrayBuffer(16);
let dv = new DataView(buf);

// Call isView with various value types
console.log(ArrayBuffer.isView(dv));
console.log(ArrayBuffer.isView(buf));
console.log(ArrayBuffer.isView(null));
console.log(ArrayBuffer.isView(42));
console.log(ArrayBuffer.isView("hello"));
