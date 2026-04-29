function outer() {
  let left = "ordinary-";
  let right = "closure";

  function read(suffix) {
    return left + right + suffix;
  }

  return read("-capture");
}

console.log(outer());
