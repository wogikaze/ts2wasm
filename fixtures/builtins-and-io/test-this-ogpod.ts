// Test Object.getOwnPropertyDescriptor on this.AggregateError
const d = Object.getOwnPropertyDescriptor(this, "AggregateError");
console.log(d === undefined ? "undefined" : "found: " + d.value);
