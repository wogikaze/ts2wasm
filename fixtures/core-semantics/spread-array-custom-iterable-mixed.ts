const iterable = {
  [Symbol.iterator]: function () {
    const state = { i: 0 };
    return {
      next: function () {
        state.i = state.i + 1;
        return { value: state.i, done: state.i > 2 };
      }
    };
  }
};
const arr = [0, ...iterable, 3];
console.log(arr.length);
console.log(arr[0]);
console.log(arr[1]);
console.log(arr[2]);
console.log(arr[3]);
console.log(arr[4]);
