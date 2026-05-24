function safeDivide(left: number, right: number): string {
  if (right === 0) {
    return "division blocked";
  }
  return String(left / right);
}

console.log(safeDivide(8, 2));
console.log(safeDivide(8, 0));
