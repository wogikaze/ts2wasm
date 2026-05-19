function run() {
  let source = 'function created() { return "after"; } throw new Error("direct boom")';
  try {
    eval(source);
  } catch (error) {
    console.log(error.name);
  }
  console.log(created());
}

run();
