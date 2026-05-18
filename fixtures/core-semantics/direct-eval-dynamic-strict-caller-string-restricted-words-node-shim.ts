function run() {
  "use strict";
  let keyword = "var";
  let source = "let text = '" + keyword + " arguments'; text";
  try {
    console.log(eval(source));
  } catch (error) {
    console.log(error.name);
  }
  console.log("after");
}

run();
