function check(actual: any, expected: any): void {
    if (actual !== expected) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Math.abs(-5), 5);
check(Math.abs(1073741824), 1073741824);
check(Math.abs(-1073741824), 1073741824);
console.log("done");
