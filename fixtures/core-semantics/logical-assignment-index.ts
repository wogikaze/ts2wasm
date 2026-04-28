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

function key(name) {
  console.log("dynamic-key");
  return name;
}

function countedRhs(value) {
  console.log("dynamic-rhs");
  return value;
}

let dynamic = { value: "kept", missing: false, empty: null };
console.log(dynamic[key("value")] ||= countedRhs("bad"));
console.log(dynamic.value);
console.log(dynamic[key("missing")] ||= countedRhs("filled-dynamic"));
console.log(dynamic.missing);
console.log(dynamic[key("empty")] ??= countedRhs("fallback-dynamic"));
console.log(dynamic.empty);
console.log(dynamic[key("missing")] &&= countedRhs("again-dynamic"));
console.log(dynamic.missing);
