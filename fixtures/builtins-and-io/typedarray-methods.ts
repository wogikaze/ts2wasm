const values = new Uint8Array([3, 1, 2]);

const doubled = values.map((value) => value * 2);
console.log(doubled.length);
console.log(doubled[0]);
console.log(doubled[1]);
console.log(doubled[2]);

const filtered = values.filter((value) => value > 1);
console.log(filtered.length);
console.log(filtered[0]);
console.log(filtered[1]);

values.forEach((value, index) => {
  console.log(value);
  console.log(index);
});

console.log(values.reduce((acc, value) => acc + value, 0));
console.log(values.every((value) => value > 0));
console.log(values.some((value) => value === 2));
console.log(values.find((value) => value > 1));

const sorted = new Uint8Array([3, 1, 2]);
sorted.sort((a, b) => a - b);
console.log(sorted[0]);
console.log(sorted[1]);
console.log(sorted[2]);

const filled = new Uint8Array([1, 2, 3]);
filled.fill(9);
console.log(filled[0]);
console.log(filled[1]);
console.log(filled[2]);

const sliced = values.slice(1, 3);
console.log(sliced.length);
console.log(sliced[0]);
console.log(sliced[1]);

const copied = new Uint8Array([1, 2, 3, 4]);
copied.copyWithin(0, 2);
console.log(copied[0]);
console.log(copied[1]);
console.log(copied[2]);
console.log(copied[3]);
