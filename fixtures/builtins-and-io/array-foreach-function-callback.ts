// Array.prototype.forEach with ordinary function-expression callback.
let values = ["Collator", "NumberFormat"];

values.forEach(function (value, index) {
  console.log(value);
  console.log(index);
});
