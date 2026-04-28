function rhs(value) {
  console.log("rhs");
  return value;
}

let target = { value: "kept", missing: false, empty: null };
console.log(target["value"] ||= rhs("bad"));
console.log(target.value);
console.log(target["missing"] ||= rhs("filled"));
console.log(target.missing);
console.log(target["empty"] ??= rhs("fallback"));
console.log(target.empty);
console.log(target["missing"] &&= rhs("again"));
console.log(target.missing);
