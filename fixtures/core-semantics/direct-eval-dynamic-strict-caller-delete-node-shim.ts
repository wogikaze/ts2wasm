"use strict";

function run() {
  let value = 1;
  let source = "delete value";
  try {
    eval(source);
    console.log("no-error");
  } catch (err) {
    console.log(err.name);
  }
  console.log(value);
}

run();
