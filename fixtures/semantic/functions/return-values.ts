// Return values from functions
function getTrue(): boolean {
  return true;
}
if (getTrue()) {
  console.log("true");
}

function getNumber(): number {
  return 99;
}
console.log(getNumber());

function getString(): string {
  return "world";
}
console.log(getString());

// Void return (no return)
function noop(): void {
  let x = 1;
  x = x + 1;
}
noop();
console.log("after void");
