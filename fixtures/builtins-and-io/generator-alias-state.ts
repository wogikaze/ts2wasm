function* gen() {
  yield 1;
  yield 2;
}

const original = gen();
const alias = original;

const first = original.next();
console.log(first.value);
console.log(first.done);

const second = alias.next();
console.log(second.value);
console.log(second.done);

const third = original.next();
console.log(third.value);
console.log(third.done);
