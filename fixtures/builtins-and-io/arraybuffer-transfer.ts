const buf = new ArrayBuffer(4);
const view = new DataView(buf);

view.setUint8(0, 11);
view.setUint8(1, 22);
view.setUint8(2, 33);
view.setUint8(3, 44);

const moved = buf.transfer(4);
const movedView = new DataView(moved);

console.log(movedView.getUint8(0));
console.log(movedView.getUint8(1));
console.log(movedView.getUint8(2));
console.log(movedView.getUint8(3));
