function* gen() {
  let value = 1;
  yield value;
  value = value + 1;
  yield value;
}

const g = gen();
const first = g.next();
console.log(first.value);
console.log(first.done);

const second = g.next();
console.log(second.value);
console.log(second.done);

const third = g.next();
console.log(third.value);
console.log(third.done);
