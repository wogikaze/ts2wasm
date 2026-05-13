function* loopGen() {
  for (let i = 0; i < 3; i = i + 1) {
    yield i;
  }
}

const g = loopGen();

let result = g.next();
console.log(result.value);
console.log(result.done);

result = g.next();
console.log(result.value);
console.log(result.done);

result = g.next();
console.log(result.value);
console.log(result.done);

result = g.next();
console.log(result.value);
console.log(result.done);

