// Mutable capture in escaping closure
function makeCounter(): () => number {
  let count = 0;
  return () => {
    count = count + 1;
    return count;
  };
}
const c = makeCounter();
console.log(c());
console.log(c());
console.log(c());
