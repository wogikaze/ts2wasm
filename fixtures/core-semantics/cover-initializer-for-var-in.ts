// Cover initializer: for (var x = y in obj)
const obj = { a: 1, b: 2, c: 3 };
for (var i = 0 in obj) {
  console.log(i);
}
