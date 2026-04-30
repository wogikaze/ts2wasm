function scale(value) {
  return this.multiplier * value;
}

let doubled = [1, 2, 3].map(scale, { multiplier: 2 });

console.log(doubled.length);
console.log(doubled[0]);
console.log(doubled[1]);
console.log(doubled[2]);
