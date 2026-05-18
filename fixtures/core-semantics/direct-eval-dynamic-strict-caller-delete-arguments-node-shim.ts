"use strict";

function run() {
  let source = "delete arguments";
  try {
    eval(source);
    console.log("no-error");
  } catch (err) {
    console.log(err.name);
  }
  console.log(arguments[0]);
}

run(9);
