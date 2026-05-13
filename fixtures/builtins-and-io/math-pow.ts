function check(actual: any, expected: any): void {
    if (actual !== expected) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

function checkNaN(actual: any): void {
    if (!Number.isNaN(actual)) {
        throw new Error(`Expected NaN, got ${actual}`);
    }
}

check(Math.pow(2, 3), 8);
check(Math.pow(1073741824, 1), 1073741824);
checkNaN(Math.pow({} as any, 2));
checkNaN(Math.pow(2, {} as any));
checkNaN(Math.pow(undefined as any, 2));
console.log("done");
