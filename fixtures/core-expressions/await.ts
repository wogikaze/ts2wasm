async function test() {
    const x = await Promise.resolve(42);
    console.log(x);
}
test();
