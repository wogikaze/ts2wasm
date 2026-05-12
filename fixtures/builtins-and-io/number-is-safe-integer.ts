// Number.isSafeInteger

function check(actual: any, expected: any): void {
    const pass = actual === expected;
    if (!pass) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Number.isSafeInteger(0), true);
check(Number.isSafeInteger(1), true);
check(Number.isSafeInteger(-1), true);
check(Number.isSafeInteger(42), true);
check(Number.isSafeInteger(3.14), false);
check(Number.isSafeInteger(NaN), false);
check(Number.isSafeInteger(Infinity), false);
check(Number.isSafeInteger(-Infinity), false);
check(Number.isSafeInteger(undefined as any), false);
check(Number.isSafeInteger(null as any), false);
check(Number.isSafeInteger("42" as any), false);
check(Number.isSafeInteger(true as any), false);
console.log("done");
