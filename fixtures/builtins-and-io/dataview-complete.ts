// DataView complete get/set methods including BigInt variants
function testBasicMethods(): void {
    const buf = new ArrayBuffer(32);
    const dv = new DataView(buf);

    // Int8
    dv.setInt8(0, -128);
    console.log(dv.getInt8(0));
    dv.setInt8(1, 127);
    console.log(dv.getInt8(1));

    // Uint8
    dv.setUint8(2, 255);
    console.log(dv.getUint8(2));
    dv.setUint8(3, 0);
    console.log(dv.getUint8(3));

    // Int16 little-endian
    dv.setInt16(4, -32768, true);
    console.log(dv.getInt16(4, true));
    dv.setInt16(6, 32767, true);
    console.log(dv.getInt16(6, true));

    // Uint16 big-endian
    dv.setUint16(8, 65535, false);
    console.log(dv.getUint16(8, false));
    dv.setUint16(10, 0, false);
    console.log(dv.getUint16(10, false));

    // Int32 little-endian
    dv.setInt32(12, -32768, true);
    console.log(dv.getInt32(12, true));
    dv.setInt32(16, 32767, true);
    console.log(dv.getInt32(16, true));
}

function testBigIntMethods(): void {
    const buf = new ArrayBuffer(32);
    const dv = new DataView(buf);

    // BigInt64 little-endian: zero
    dv.setBigInt64(0, 0n, true);
    console.log(dv.getBigInt64(0, true));

    // BigInt64 little-endian: positive
    dv.setBigInt64(0, 42n, true);
    console.log(dv.getBigInt64(0, true));

    // BigInt64 little-endian: negative
    dv.setBigInt64(0, -1n, true);
    console.log(dv.getBigInt64(0, true));

    // BigInt64 little-endian: large positive
    dv.setBigInt64(0, 65536n, true);
    console.log(dv.getBigInt64(0, true));

    // BigUint64 little-endian
    dv.setBigUint64(8, 100n, true);
    console.log(dv.getBigUint64(8, true));

    dv.setBigUint64(8, 0n, true);
    console.log(dv.getBigUint64(8, true));

    // BigUint64 big-endian
    dv.setBigUint64(8, 100n, false);
    console.log(dv.getBigUint64(8, false));
}

testBasicMethods();
testBigIntMethods();
