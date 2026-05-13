let value = 7;
let key = "computed";
let object = { value, [key]: 11, method() { return value; } };
console.log(object.value);
console.log(object["computed"]);
