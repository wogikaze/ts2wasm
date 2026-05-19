function run() {
  let source = 'var created = "after"; throw new Error("direct boom")';
  try {
    eval(source);
  } catch (error) {
    console.log(error.name);
  }
  let read = "created";
  console.log(eval(read));
}

run();
