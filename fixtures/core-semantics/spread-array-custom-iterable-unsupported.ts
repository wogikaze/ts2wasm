const iterable = {
  [Symbol.iterator]: function () {
    return {
      next: function () {
        return { value: 1, done: true };
      }
    };
  }
};

const arr = [...iterable];
console.log(arr.length);
