const iterable = {
  [Symbol.iterator]: function () {
    return {
      next: function () {
        return { value: undefined, done: true };
      }
    };
  }
};
const arr = [...iterable];
console.log(arr.length);
