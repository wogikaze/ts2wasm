function* gen() {
  yield 1;
  yield 2;
}

const arr = [...gen()];
console.log(arr.length);
console.log(arr[0]);
console.log(arr[1]);
