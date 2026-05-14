function revive(key, value) {
  if (key === "0") {
    return "first";
  }
  if (key === "n") {
    return value + 1;
  }
  if (key === "drop") {
    return undefined;
  }
  return value;
}

let obj = JSON.parse('{"n":1,"drop":2,"items":[1,2]}', revive);
console.log(JSON.stringify(obj));
console.log(JSON.stringify(obj.items));
