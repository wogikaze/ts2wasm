"use strict";

function run() {
  let source = "var arguments = 1";
  try {
    eval(source);
    console.log("no-error");
  } catch (err) {
    console.log(err.name);
  }
  console.log(arguments[0]);
}

run(9);
