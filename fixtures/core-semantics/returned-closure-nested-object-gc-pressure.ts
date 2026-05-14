// Returned closure captures an object containing an array, invoked after GC pressure.
// This tests that the closure's capture slots are correctly marked by the GC scanner
// so the nested object and its array payload survive allocation pressure.

function makeProcessor() {
  let state = {
    items: ["initial", "nested"],
    counter: 42,
  };

  function process(tag: string) {
    return state.items[0] + ":" + tag + ":" + state.counter;
  }

  return process;
}

let proc = makeProcessor();
let i = 0;
let s = "";

// Allocation pressure to trigger GC
while (i < 3000) {
  s = "pressure-" + i;
  i = i + 1;
}

console.log(proc("after-pressure"));
