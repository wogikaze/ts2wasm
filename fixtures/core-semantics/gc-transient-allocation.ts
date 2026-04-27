let i = 0;
let total = 0;
let s = "";

while (i < 5000) {
  s = "gc-item-" + i;
  total = total + 1;
  i = i + 1;
}

console.log(total);
