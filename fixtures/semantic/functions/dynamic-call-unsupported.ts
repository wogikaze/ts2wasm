// Dynamic calls not supported by current HIR lowering
// Reassigning a function-valued variable prevents static resolution

function compute(n: number): number {
  return n * 2;
}

let fn = compute;
fn = 5;
// This call should fail: function-valued local after reassignment
console.log(fn(3));
