function run() {
  "use strict";
  let source = "void /var arguments/.source";
  try {
    console.log(eval(source) === undefined);
  } catch (error) {
    console.log(error.name);
  }
  console.log("after");
}

run();
