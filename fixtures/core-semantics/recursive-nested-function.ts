function run() {
  function fact(n) {
    if (n === 1) {
      return 1;
    }
    return fact(n - 1) * n;
  }

  return fact(5);
}

console.log(run());
