"use strict";

function run() {
  let create = "var hidden = 7; hidden";
  let read = "typeof hidden";
  console.log(eval(create));
  console.log(eval(read));
}

run();
