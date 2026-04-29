let fact = function fact(n) {
  if (n === 1) {
    return 1;
  }
  return fact(n - 1) * n;
};

console.log(fact(3));
