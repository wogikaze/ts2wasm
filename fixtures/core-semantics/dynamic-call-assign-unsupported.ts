// Dynamic call through variable reassignment
// When a function reference is stored in a local variable that gets reassigned,
// the compiler cannot statically resolve the call target.

function greet(): void {
  console.log("hello");
}

function farewell(): void {
  console.log("goodbye");
}

let fn = greet;
fn = farewell;
fn();
