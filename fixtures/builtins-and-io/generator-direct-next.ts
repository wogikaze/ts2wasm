function* gen() {
  console.log("body");
  yield 1;
}

const first = gen().next();
console.log(first.value);
console.log(first.done);

const second = gen().next();
console.log(second.value);
console.log(second.done);
