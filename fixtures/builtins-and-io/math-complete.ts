// Math static functions with exact integer-backed results.
function check(actual: any, expected: any): void {
    if (actual !== expected) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(Math.abs(-5), 5);
check(Math.acos(1), 0);
check(Math.acosh(1), 0);
check(Math.asin(0), 0);
check(Math.asinh(0), 0);
check(Math.atan(0), 0);
check(Math.atan2(0, 1), 0);
check(Math.atanh(0), 0);
check(Math.cbrt(27), 3);
check(Math.ceil(3), 3);
check(Math.clz32(1), 31);
check(Math.cos(0), 1);
check(Math.cosh(0), 1);
check(Math.exp(0), 1);
check(Math.expm1(0), 0);
check(Math.floor(3), 3);
check(Math.fround(7), 7);
check(Math.hypot(3, 4), 5);
check(Math.imul(7, 6), 42);
check(Math.log(1), 0);
check(Math.log10(1), 0);
check(Math.log1p(0), 0);
check(Math.log2(1), 0);
check(Math.max(1, 5, 3), 5);
check(Math.min(1, 5, 3), 1);
check(Math.pow(2, 3), 8);
check(Math.round(3), 3);
check(Math.sign(-3), -1);
check(Math.sin(0), 0);
check(Math.sinh(0), 0);
check(Math.sqrt(144), 12);
check(Math.tan(0), 0);
check(Math.tanh(0), 0);
check(Math.trunc(42), 42);
console.log("done");
