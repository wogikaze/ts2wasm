function clampScore(score: number): string {
  if (score < 0) {
    return "below";
  }
  if (score > 100) {
    return "above";
  }
  if (score === 0 || score === 100) {
    return "boundary";
  }
  return "inside";
}

console.log(clampScore(-1));
console.log(clampScore(0));
console.log(clampScore(50));
console.log(clampScore(100));
console.log(clampScore(101));
