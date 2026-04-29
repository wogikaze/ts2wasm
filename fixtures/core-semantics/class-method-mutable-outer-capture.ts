var callCount = 0;

class C {
  method() {
    callCount = callCount + 1;
  }
}

let c = new C();
c.method();
console.log(callCount);
