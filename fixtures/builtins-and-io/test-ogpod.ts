// Test Object.getOwnPropertyDescriptor on AggregateError sentinel
const d = Object.getOwnPropertyDescriptor(AggregateError, "name");
console.log(d.value === "AggregateError");
console.log(d.writable === false);
console.log(d.enumerable === false);
console.log(d.configurable === true);
