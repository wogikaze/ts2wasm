// Closure GC survival: verify that closure capture slots are properly marked
// under GC pressure. If the GC's mark loop does not scan captured locals,
// the closure will alias a freed or overwritten heap slot.

function makeCounter(): () => number[] {
  let arr: number[] = [1, 2, 3];
  return (): number[] => {
    return arr;
  };
}

let counter: () => number[] = makeCounter();

// Allocation pressure that could overwrite the captured array's slot
let i: number = 0;
let junk: string = "";
while (i < 5000) {
  junk = "junk-" + i;
  i = i + 1;
}

let result: number[] = counter();
console.log(result[0] === 1 ? "pass" : "fail");
console.log(result.length === 3 ? "pass" : "fail");
