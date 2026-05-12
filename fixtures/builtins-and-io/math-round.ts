function check(actual: any, expected: any): void {
    if (actual !== expected) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Math.round(3), 3);
check(Math.round(1073741824), 1073741824);
check(Math.round(-1073741824), -1073741824);
console.log("done");
