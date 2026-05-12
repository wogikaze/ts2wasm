// Pure try-finally (no catch) — exercises the TryFinally IR variant.
function test(): number {
  let x = 1;
  try {
    x = 2;
  } finally {
    x = 3;
  }
  return x;
}
console.log(test());
