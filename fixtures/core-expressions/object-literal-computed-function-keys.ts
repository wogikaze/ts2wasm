function f() {}
function* g() {
  return 1;
}

const fnObject: any = {
  [function () {}]: "fn",
};
console.log(fnObject[function () {}]);
console.log(fnObject[String(function () {})]);

const arrowObject: any = {
  [() => {}]: "arrow",
};
console.log(arrowObject[() => {}]);
console.log(arrowObject[String(() => {})]);

const asyncArrowObject: any = {
  [async () => {}]: "async",
};
console.log(asyncArrowObject[async () => {}]);
console.log(asyncArrowObject[String(async () => {})]);

const undefinedObject: any = {
  [f()]: "undefined",
};
console.log(undefinedObject[f()]);
console.log(undefinedObject[String(f())]);

const generatorObject: any = {
  [g()]: "generator",
};
console.log(generatorObject[g()]);
console.log(generatorObject[String(g())]);
