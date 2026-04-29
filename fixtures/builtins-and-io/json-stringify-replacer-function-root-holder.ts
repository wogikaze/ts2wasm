function rootHolder(key, value) {
  if (key === "") {
    return this[""];
  }
  return value;
}

console.log(JSON.stringify({ a: 1 }, rootHolder));
