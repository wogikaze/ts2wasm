function* gen() {
  console.log("body");
  yield 1;
}

const g = gen();
console.log("created");
const first = g.next();
console.log(first.value);
console.log(first.done);
