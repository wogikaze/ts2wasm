function run() {
  let value = 1;
  let result = eval('"use strict"; let value = 2; value');
  console.log(result);
  console.log(value);
}

run();
