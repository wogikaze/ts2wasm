let doubled = [1, 2, 3].map(function (value) {
  return this.multiplier * value;
}, { multiplier: 2 });

console.log(doubled.length);
console.log(doubled[0]);
console.log(doubled[1]);
console.log(doubled[2]);
