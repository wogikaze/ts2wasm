function* gen() {
  console.log("first step");
  yield 1;
  console.log("second step");
  yield 2;
}

const g = gen();
console.log("created");

const first = g.next();
console.log(first.value);
console.log(first.done);

const second = g.next();
console.log(second.value);
console.log(second.done);
