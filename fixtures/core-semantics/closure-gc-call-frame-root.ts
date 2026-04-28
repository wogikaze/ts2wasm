function runClosure() {
  let holder = { keep: "closure-frame-alive" };
  const read = () => holder.keep;
  let i = 0;
  let s = "";

  while (i < 2200) {
    s = "closure-gc-" + i;
    i = i + 1;
  }

  return read();
}

console.log(runClosure());
