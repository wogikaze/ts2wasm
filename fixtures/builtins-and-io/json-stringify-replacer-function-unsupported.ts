function keep(key, value) {
  return value;
}

console.log(JSON.stringify({ a: 1 }, keep));
