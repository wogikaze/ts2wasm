function run() {
  let value = "before";
  let source = 'value = "after"; throw new Error("direct boom")';
  try {
    eval(source);
  } catch (error) {
    console.log(error.name);
  }
  console.log(value);
}

run();
