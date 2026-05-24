function fareBucket(age: number): string {
  if (age < 0) {
    return "invalid";
  }
  if (age < 13) {
    return "child";
  }
  if (age < 65) {
    return "adult";
  }
  return "senior";
}

console.log(fareBucket(-1));
console.log(fareBucket(8));
console.log(fareBucket(40));
console.log(fareBucket(70));
