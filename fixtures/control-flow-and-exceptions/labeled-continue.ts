let outer = 0;
let visited = 0;
let afterInner = 0;

outerLoop:
while (outer < 3) {
  outer = outer + 1;
  let inner = 0;
  while (inner < 3) {
    inner = inner + 1;
    if (inner === 2) {
      continue outerLoop;
    }
    visited = visited + 1;
  }
  afterInner = afterInner + 1;
}

console.log(outer);
console.log(visited);
console.log(afterInner);
