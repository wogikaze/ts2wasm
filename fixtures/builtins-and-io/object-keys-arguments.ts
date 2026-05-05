function test(x: number, y: number, z: number): boolean {
  var a = Object.keys(arguments);
  console.log(a.length);
  console.log(a[0]);
  console.log(a[1]);
  console.log(a[2]);
  return a.length === 3;
}
var result = test(1, 2, 3);
console.log(result);
