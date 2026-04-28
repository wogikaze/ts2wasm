function rhs(value) {
  console.log("rhs");
  return value;
}

let target = { value: "kept", missing: false };
console.log(target.value ||= rhs("bad"));
console.log(target.value);
console.log(target.missing ||= rhs("filled"));
console.log(target.missing);

function getOrSkip() {
  console.log("receiver");
  return { value: "kept" };
}
console.log(getOrSkip().value ||= rhs("bad"));

function getOrRun() {
  console.log("receiver");
  return { value: false };
}
console.log(getOrRun().value ||= rhs("filled-member"));

function getAndSkip() {
  console.log("receiver");
  return { value: false };
}
console.log(getAndSkip().value &&= rhs("bad"));

function getAndRun() {
  console.log("receiver");
  return { value: true };
}
console.log(getAndRun().value &&= rhs("updated-member"));

function getNullishSkip() {
  console.log("receiver");
  return { value: "kept-nullish" };
}
console.log(getNullishSkip().value ??= rhs("bad"));

function getNullishRun() {
  console.log("receiver");
  return { value: null };
}
console.log(getNullishRun().value ??= rhs("fallback-member"));
