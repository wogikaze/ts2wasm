// ================================================================
// Bun victory benchmark tier 1 — string-concat-loop
// String concatenation in a loop building a large string.
// Bun's JIT can optimize string concatenation with rope
// representation or internal buffers, while ts2wasm's
// runtime creates many intermediate string objects.
// ================================================================
// Expected output: 50000

function Main(): void {
  const N: number = 50000;
  let s: string = "";

  for (let i = 0; i < N; i++) {
    s += "x";
  }

  console.log(String(s.length));
}
