function check(actual: boolean, expected: boolean): void {
    if (actual !== expected) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Number.isFinite(42), true);
check(Number.isFinite(0), true);
check(Number.isFinite(NaN), false);
check(Number.isFinite(Infinity), false);
check(Number.isFinite(-Infinity), false);
check(Number.isFinite(undefined as any), false);
check(Number.isFinite(null as any), false);
check(Number.isFinite("42" as any), false);
check(Number.isFinite(true as any), false);
console.log("done");
