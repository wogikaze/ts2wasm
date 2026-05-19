function run() {
  let source = "typeof later";
  try {
    eval(source);
  } catch (error) {
    console.log(error.name);
  }
  let later = "after";
  console.log(later);
}

run();
