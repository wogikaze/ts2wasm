function run() {
  let source = "later.value";
  try {
    eval(source);
  } catch (error) {
    console.log(error.name);
  }
  let later = { value: "after" };
  console.log(later.value);
}

run();
