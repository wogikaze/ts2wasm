// Number.isInteger

function check(actual: any, expected: any): void {
    const pass = actual === expected;
    if (!pass) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Number.isInteger(0), true);
check(Number.isInteger(1), true);
check(Number.isInteger(-1), true);
check(Number.isInteger(42), true);
check(Number.isInteger(-999), true);
check(Number.isInteger(3.14), false);
check(Number.isInteger(0.5), false);
check(Number.isInteger(-0.5), false);
check(Number.isInteger(1.0), true);
check(Number.isInteger(NaN), false);
check(Number.isInteger(Infinity), false);
check(Number.isInteger(-Infinity), false);
check(Number.isInteger(undefined as any), false);
check(Number.isInteger(null as any), false);
check(Number.isInteger("42" as any), false);
check(Number.isInteger(true as any), false);
check(Number.isInteger(false as any), false);
console.log("done");
