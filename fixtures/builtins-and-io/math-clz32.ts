// Math.clz32

function check(actual: any, expected: any): void {
    const pass = actual === expected;
    if (!pass) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Math.clz32(0), 32);
check(Math.clz32(1), 31);
check(Math.clz32(2), 30);
check(Math.clz32(3), 30);
check(Math.clz32(4), 29);
check(Math.clz32(8), 28);
check(Math.clz32(0x80000000), 0);
console.log("done");
