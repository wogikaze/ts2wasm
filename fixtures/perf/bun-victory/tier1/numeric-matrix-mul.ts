// ================================================================
// Bun victory benchmark tier 1 — numeric-matrix-mul
// 2D matrix multiplication (N x N) * (N x N).
// Triple-nested loop — O(N^3) — heavy on array indexing and
// arithmetic. Bun's JIT optimizes bounds checks and loop
// induction variables far beyond ts2wasm's interpreter.
// ================================================================
// Expected output: 273024000

function Main(): void {
  const N: number = 80;
  const A: number[][] = [];
  const B: number[][] = [];
  const C: number[][] = [];

  for (let i = 0; i < N; i++) {
    const rowA: number[] = [];
    const rowB: number[] = [];
    const rowC: number[] = [];
    for (let j = 0; j < N; j++) {
      rowA.push(i + j);
      rowB.push(i - j);
      rowC.push(0);
    }
    A.push(rowA);
    B.push(rowB);
    C.push(rowC);
  }

  for (let i = 0; i < N; i++) {
    for (let j = 0; j < N; j++) {
      let acc: number = 0;
      for (let k = 0; k < N; k++) {
        acc += A[i][k] * B[k][j];
      }
      C[i][j] = acc;
    }
  }

  // Checksum: sum of all elements in C
  let total: number = 0;
  for (let i = 0; i < N; i++) {
    for (let j = 0; j < N; j++) {
      total += C[i][j];
    }
  }

  console.log(String(total));
}
