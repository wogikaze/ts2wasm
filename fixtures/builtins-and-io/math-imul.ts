// Math.imul

function check(actual: any, expected: any): void {
    const pass = actual === expected;
    if (!pass) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Math.imul(0, 0), 0);
check(Math.imul(3, 4), 12);
check(Math.imul(7, 6), 42);
check(Math.imul(-3, 4), -12);
check(Math.imul(100000, 100000), 1410065408);
check(Math.imul(0x7fffffff, 2), -2);
console.log("done");
