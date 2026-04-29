let s = new Set();
s.add("a");
s.add("b");
s.add("a");
for (let value of s) {
  console.log(value);
}
