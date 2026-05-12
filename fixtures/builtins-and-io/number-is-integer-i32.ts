// Number.isInteger with i32 values that are emitted as heap-backed numbers.

function check(actual: any, expected: any): void {
    if (actual !== expected) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Number.isInteger(0), true);
check(Number.isInteger(2147483647), true);
check(Number.isInteger(1073741824), true);
check(Number.isInteger(-1073741824), true);
check(Number.isInteger(undefined as any), false);
check(Number.isInteger("1073741824" as any), false);
console.log("done");
