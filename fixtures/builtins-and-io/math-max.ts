function check(actual: any, expected: any): void {
    if (actual !== expected) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Math.max(1, 5), 5);
check(Math.max(1073741824, 5), 1073741824);
check(Math.max(-1073741824, 5), 5);
check(Math.max(-1073741824, -5), -5);
console.log("done");
