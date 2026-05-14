function reviver(key, value) {
  if (typeof value === "number") {
    return value * 10;
  }
  return value;
}

var arr = JSON.parse('[1,2,3]', reviver);
console.log(arr[0]);
console.log(arr[1]);
console.log(arr[2]);
