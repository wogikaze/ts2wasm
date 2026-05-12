function check(actual: any, expected: any): void {
    if (actual !== expected) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Math.min(1, 5), 1);
check(Math.min(1073741824, 5), 5);
check(Math.min(-1073741824, 5), -1073741824);
check(Math.min(-1073741824, -5), -1073741824);
console.log("done");
