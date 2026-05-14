function restArgumentsObject(x, ...y) {
  console.log(arguments.length);
  console.log(arguments[0]);
  console.log(arguments[1]);
  console.log(arguments[2]);
  console.log(y.length);
  console.log(y[0]);
  console.log(y[1]);
}

restArgumentsObject(1, 2, 3);
