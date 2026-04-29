var initCount = 0;
function counter() {
  initCount += 1;
}

var C = class {
  method([value = counter()]) {
    return value;
  }
};

new C().method([undefined]);
