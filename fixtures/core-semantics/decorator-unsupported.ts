// Decorator — should produce unsupported diagnostic
function sealed(target: any) {}

@sealed
class MyClass {
  constructor() {
    console.log("decorated");
  }
}
new MyClass();
