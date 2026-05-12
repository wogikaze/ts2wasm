// Math.cbrt

function check(actual: any, expected: any): void {
    const pass = actual === expected;
    if (!pass) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Math.cbrt(0), 0);
check(Math.cbrt(1), 1);
check(Math.cbrt(8), 2);
check(Math.cbrt(27), 3);
check(Math.cbrt(64), 4);
check(Math.cbrt(125), 5);
check(Math.cbrt(-1), -1);
check(Math.cbrt(-8), -2);
check(Math.cbrt(-27), -3);
console.log("done");
