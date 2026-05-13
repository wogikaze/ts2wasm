function* gen() {
  console.log("body");
  yield 1;
  yield 2;
}

const first = gen();
const second = gen();

let result = first.next();
console.log(result.value);
console.log(result.done);

result = second.next();
console.log(result.value);
console.log(result.done);

result = first.next();
console.log(result.value);
console.log(result.done);

result = second.next();
console.log(result.value);
console.log(result.done);
