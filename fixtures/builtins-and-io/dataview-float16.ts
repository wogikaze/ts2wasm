// DataView getFloat16/setFloat16 — half-precision float (binary16) methods.
// These are part of the Float16Array proposal (ES2025+ / Stage 4).
function testSetGetFloat16(): void {
    const buf = new ArrayBuffer(8);
    const view = new DataView(buf);

    // setFloat16(byteOffset, value, littleEndian?)
    // getFloat16(byteOffset, littleEndian?)

    // Float16: 1.5 (0x3E00 in f16)
    view.setFloat16(0, 1.5, true);
    console.log(view.getFloat16(0, true));

    // Float16: 0.0
    view.setFloat16(2, 0, true);
    console.log(view.getFloat16(2, true));

    // Float16: -2.0
    view.setFloat16(4, -2, true);
    console.log(view.getFloat16(4, true));

    // Float16: large endian
    view.setFloat16(0, 1.5, false);
    console.log(view.getFloat16(0, false));

    // Float16: zero with big-endian
    view.setFloat16(2, 0, false);
    console.log(view.getFloat16(2, false));

    // Float16: negative with big-endian
    view.setFloat16(4, -2, false);
    console.log(view.getFloat16(4, false));
}

function testFloat16EdgeCases(): void {
    const buf = new ArrayBuffer(8);
    const view = new DataView(buf);

    // Positive infinity in f16 = 0x7C00
    view.setFloat16(0, Infinity, true);
    console.log(view.getFloat16(0, true));

    // Negative infinity in f16 = 0xFC00
    view.setFloat16(2, -Infinity, true);
    console.log(view.getFloat16(2, true));

    // NaN in f16 = 0x7E00
    view.setFloat16(4, NaN, true);
    console.log(view.getFloat16(4, true));

    // Large values that overflow f16 range → Infinity in f16
    view.setFloat16(0, 65504, true);
    console.log(view.getFloat16(0, true));

    // Subnormal? Try a very small value
    view.setFloat16(2, 0.00006103515625, true);
    console.log(view.getFloat16(2, true));
}

testSetGetFloat16();
testFloat16EdgeCases();
