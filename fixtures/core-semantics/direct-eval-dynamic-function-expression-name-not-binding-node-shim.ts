function run() {
  let source = "delete globalThis.hiddenProbe; let holder = function hiddenProbe() { return 1; }; hiddenProbe = 2; globalThis.hiddenProbe";
  let read = "globalThis.hiddenProbe";
  console.log(eval(source));
  console.log(eval(read));
}

run();
