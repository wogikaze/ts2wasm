// Math.sqrt
function check(actual: any, expected: any): void {
    if (actual !== expected) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Math.sqrt(144), 12);
check(Math.sqrt(1073741824), 32768);
console.log("done");
