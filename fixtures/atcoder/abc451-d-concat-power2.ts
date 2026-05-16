function Main(): void {
    const N = 5;
    const powersOfTwo: number[] = [];
    for (let i = 0; 2 ** i <= 1000000000; i++) {
        powersOfTwo.push(2 ** i);
    }
    const strArr: string[] = powersOfTwo.map(n => String(n));

    // Iterative DFS (avoids WASM operand stack overflow from recursion)
    const all: number[] = [];
    const stack: string[] = [""];
    while (stack.length > 0) {
        const before = stack.pop()!;
        if (before.length > 0) all.push(+before);
        const remainDigits = 5 - before.length;
        for (let i = 0; i < strArr.length; i++) {
            const after = strArr[i];
            if (after.length > remainDigits) break;
            stack.push(before + after);
        }
    }

    // Manual dedup after sort (avoids Set/spread operator WASM issues)
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
