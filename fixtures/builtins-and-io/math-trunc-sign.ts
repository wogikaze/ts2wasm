// Math.trunc and Math.sign

function check(actual: any, expected: any): void {
    const pass = actual === expected;
    if (!pass) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

// Math.trunc — no-op for integers, returns integers unchanged
check(Math.trunc(42), 42);
check(Math.trunc(0), 0);
check(Math.trunc(-7), -7);

// Math.sign
check(Math.sign(5), 1);
check(Math.sign(0), 0);
check(Math.sign(-3), -1);
