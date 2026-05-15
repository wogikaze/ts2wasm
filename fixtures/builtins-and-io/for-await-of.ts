// for-await-of iteration protocol (issue I-20260513-EGPKBA)
// Build-smoke test: async iteration syntax
async function test() {
  var items: number[] = [1, 2, 3];
  for await (let x of items) {
    console.log(x);
  }
}
test();
