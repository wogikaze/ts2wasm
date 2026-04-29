function dropB(key, value) {
  if (key === "b") {
    return undefined;
  }
  return value;
}

console.log(JSON.stringify({ a: 1, b: 2 }, dropB));
