const obj = { a: 1 };
const d = Object.getOwnPropertyDescriptor(obj, "a");
console.log(d.value);
console.log(d.writable);
console.log(d.enumerable);
console.log(d.configurable);
console.log(d.get);
console.log(d.set);
const missing = Object.getOwnPropertyDescriptor(obj, "b");
if (missing === undefined) {
  console.log("missing");
} else {
  console.log(missing.value);
}
