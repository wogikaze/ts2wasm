const buf = new ArrayBuffer(8);
const transferred = buf.transfer(4);
const view = new DataView(transferred);
view.setInt8(0, 7);
console.log(ArrayBuffer.isView(view));
console.log(view.getInt8(0));

const typed = new Uint8Array([1, 2]);
console.log(ArrayBuffer.isView(typed));

const shared = new SharedArrayBuffer(4);
const sharedView = new DataView(shared);
sharedView.setInt8(0, 3);
console.log(sharedView.getInt8(0));
console.log(ArrayBuffer.isView(shared));
