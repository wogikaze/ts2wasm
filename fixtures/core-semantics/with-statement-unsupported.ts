// with statement — should produce precise unsupported diagnostic
const obj = { x: 10, y: 20 };
with (obj) {
  console.log(x + y);
}
