function run() {
  "use strict";
  let source = "/var arguments/.source";
  try {
    console.log(eval(source));
  } catch (error) {
    console.log(error.name);
  }
  console.log("after");
}

run();
