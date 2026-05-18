"use strict";

function run() {
  let source = "const {value: eval} = {value: 1}";
  try {
    eval(source);
    console.log("no-error");
  } catch (err) {
    console.log(err.name);
  }
  console.log("after");
}

run();
