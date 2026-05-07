const iterable = {
  [Symbol.iterator]: function () {
    const state = { i: 0 };
    return {
      next: function () {
        state.i = state.i + 1;
        return { value: state.i * 10, done: state.i > 3 };
      }
    };
  }
};
const arr = [...iterable];
console.log(arr.length);
console.log(arr[0]);
console.log(arr[1]);
console.log(arr[2]);
