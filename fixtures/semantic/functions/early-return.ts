// Early return from functions
function firstPositive(a: number, b: number): number {
  if (a > 0) {
    return a;
  }
  return b;
}
console.log(firstPositive(3, 5).toString());
console.log(firstPositive(-1, 5).toString());

function classify(n: number): string {
  if (n > 0) {
    return "positive";
  }
  if (n < 0) {
    return "negative";
  }
  return "zero";
}
console.log(classify(5));
console.log(classify(-3));
console.log(classify(0));
