function ignored(key, value) {
  return "not used";
}

console.log(JSON.stringify({ a: 1, b: 2 }, [true, false, null, undefined, ignored, Number, String, (key, value) => value, "b"]));
console.log(JSON.stringify({ a: 1, b: 2 }, [new Boolean(true), new Object(), { ignored: true }, Symbol("x"), "a"]));
