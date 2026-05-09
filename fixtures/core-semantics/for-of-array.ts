// for...of on array (requires iterator protocol)
const arr = [10, 20, 30];
let sum = 0;
for (const val of arr) {
  sum = sum + val;
}
console.log(sum);
