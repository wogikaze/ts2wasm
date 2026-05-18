function f(seed) {
  let value = seed;
  let source = "value = value + arguments[0]; value";
  let run = () => eval(source);
  console.log(run());
  console.log(value);
}

f(3);
