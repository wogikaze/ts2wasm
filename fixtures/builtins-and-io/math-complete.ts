function check(actual: any, expected: any): void {
    const pass = actual === expected;
    if (!pass) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Math.floor(3), 3);
check(Math.ceil(-3), -3);
check(Math.round(1073741824), 1073741824);
check(Math.abs(-1073741824), 1073741824);
check(Math.max(1, 5), 5);
check(Math.max(1, 5, 3), 5);
check(Math.min(1, 5), 1);
check(Math.min(1, 5, -2), -2);
check(Math.pow(2, 3), 8);
check(Math.trunc(-7), -7);
check(Math.sign(-3), -1);
check(Math.cbrt(27), 3);
check(Math.clz32(1), 31);
check(Math.imul(7, 6), 42);
check(Math.sqrt(144), 12);

console.log("done");
