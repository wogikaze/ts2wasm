function keepObject() {
  let holder = { keep: "function" };
  let i = 0;
  let s = "";

  while (i < 2500) {
    s = "gc-high-pressure-function-" + i;
    i = i + 1;
  }

  return holder.keep;
}

let holder = { keep: "top" };
let i = 0;
let s = "";

while (i < 2500) {
  s = "gc-high-pressure-top-" + i;
  i = i + 1;
}

console.log(holder.keep + ":" + keepObject());
