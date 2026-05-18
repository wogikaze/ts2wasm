function f() {
  let source =
    "globalThis.__ts2wasmDirectEvalObject = globalThis.__ts2wasmDirectEvalObject || { value: 7 }; globalThis.__ts2wasmDirectEvalObject";
  console.log(eval(source) === eval(source));
  console.log(eval(source).value);
}

f();
