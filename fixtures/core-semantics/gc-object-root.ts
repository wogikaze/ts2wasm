function keepObject() {
  let holder = { keep: "function" };
  let i = 0;
  let s = "";

  while (i < 2000) {
    s = "gc-function-root-" + i;
    i = i + 1;
  }

  return holder.keep;
}

let holder = { keep: "top" };
let i = 0;
let s = "";

while (i < 2000) {
  s = "gc-top-root-" + i;
  i = i + 1;
}

console.log(holder.keep + ":" + keepObject());
