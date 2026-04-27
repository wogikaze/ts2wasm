let value = 0;

target:
if (true) {
  value = 1;
  break target;
  value = 2;
}

console.log(value);
