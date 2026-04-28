function rhs(value) {
  console.log("rhs");
  return value;
}

let target = { value: "kept", missing: false };
console.log(target.value ||= rhs("bad"));
console.log(target.value);
console.log(target.missing ||= rhs("filled"));
console.log(target.missing);
