var obj = {
  *foo(a) {
    yield a + 1;
    return;
  },
};

var g = obj.foo(3);
var first = g.next();
console.log(first.value);
console.log(first.done);

var second = g.next();
console.log(second.value);
console.log(second.done);
