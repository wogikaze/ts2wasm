// AtCoder ABC 451 D - Concat Power of 2
// WASM-adapted version: iterative DFS, manual dedup
function Main(): void {
    const N = 69;
    const powersOfTwo: number[] = [];
    for (let i = 0; 2 ** i <= 1e9; i++) {
        powersOfTwo.push(2 ** i);
    }
    const powersOfTwoStr: string[] = powersOfTwo.map(n => String(n));

    // Iterative DFS
    const all: number[] = [];
    const stack: string[] = [""];
    while (stack.length > 0) {
        const before = stack.pop()!;
        if (before.length > 0) all.push(+before);
        const remainDigits = 7 - before.length;
        for (let i = 0; i < powersOfTwoStr.length; i++) {
            const after = powersOfTwoStr[i];
            if (after.length > remainDigits) break;
            stack.push(before + after);
        }
    }

    // Manual dedup after sort
    all.sort((a, b) => a - b);
    const deduped: number[] = [];
    for (let i = 0; i < all.length; i++) {
        if (i === 0 || all[i] !== all[i - 1]) {
            deduped.push(all[i]);
        }
    }
    console.log(String(deduped[N - 1]));
}

Main();
