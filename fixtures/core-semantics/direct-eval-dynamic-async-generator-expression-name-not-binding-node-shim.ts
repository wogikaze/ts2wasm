function run() {
  let asyncSource = "delete globalThis.asyncHiddenProbe; let holder = async function asyncHiddenProbe() { return 1; }; asyncHiddenProbe = 3; globalThis.asyncHiddenProbe";
  let asyncRead = "globalThis.asyncHiddenProbe";
  let generatorSource = "delete globalThis.generatorHiddenProbe; let holder = function* generatorHiddenProbe() { yield 1; }; generatorHiddenProbe = 4; globalThis.generatorHiddenProbe";
  let generatorRead = "globalThis.generatorHiddenProbe";
  console.log(eval(asyncSource));
  console.log(eval(asyncRead));
  console.log(eval(generatorSource));
  console.log(eval(generatorRead));
}

run();
