"use strict";

function run() {
  let source = "async function eval() {}";
  try {
    eval(source);
    console.log("no-error");
  } catch (err) {
    console.log(err.name);
  }
  console.log("after");
}

run();
