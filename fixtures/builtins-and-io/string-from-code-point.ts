// String.fromCodePoint

function check(actual: any, expected: any): void {
    const pass = actual === expected;
    if (!pass) {
        throw new Error(`Expected ${expected}, got ${actual}`);
    }
}

check(String.fromCodePoint(65), "A");
check(String.fromCodePoint(97), "a");
check(String.fromCodePoint(48), "0");
check(String.fromCodePoint(32), " ");
check(String.fromCodePoint(0x24), "$");
check(String.fromCodePoint(0x41), "A");
check(String.fromCodePoint(0x60), "`");
check(String.fromCodePoint(0x7A), "z");
check(String.fromCodePoint(0x80), "\u0080");
check(String.fromCodePoint(0xA9), "\u00A9");
check(String.fromCodePoint(0xFF), "\u00FF");
check(String.fromCodePoint(0x100), "\u0100");
console.log("done");
