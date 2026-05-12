function check(actual: boolean, expected: boolean): void {
    if (actual !== expected) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Number.isNaN(NaN), true);
check(Number.isNaN(42), false);
check(Number.isNaN(Infinity), false);
check(Number.isNaN(-Infinity), false);
check(Number.isNaN(undefined as any), false);
check(Number.isNaN(null as any), false);
check(Number.isNaN("hello" as any), false);
check(Number.isNaN("42" as any), false);
console.log("done");
