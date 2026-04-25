// Test fixture: should fail to compile (uses unsupported async syntax)
async function test() {
    await Promise.resolve();
}
test();
