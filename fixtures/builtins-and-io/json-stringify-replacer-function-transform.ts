function transform(key, value) {
  if (key === "a") {
    return "one";
  }
  return value;
}

console.log(JSON.stringify({ a: 1, b: 2 }, transform));
