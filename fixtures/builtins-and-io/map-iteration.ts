let m = new Map();
m.set("a", 1);
m.set("b", 2);
m.set("a", 3);

for (let entry of m) {
  console.log(entry[0]);
  console.log(entry[1]);
}
