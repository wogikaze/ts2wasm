var callCount = 0;

var obj = {
  method(a: number) {
    console.log(a);
    callCount = callCount + 1;
  },
};

obj.method(42);
console.log(callCount);
