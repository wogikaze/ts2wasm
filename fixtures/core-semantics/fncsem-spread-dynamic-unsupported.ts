function values() {
  return "345";
}

function sum(left, middle, right) {
  return left + middle + right;
}

console.log(sum(...values()));
