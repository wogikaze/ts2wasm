function run() {
  let fallback = "fallback";
  eval('for (var [first = fallback] of [[undefined]]) {}');
  console.log(first);

  let next = "next";
  eval('for (var { value = next } of [{}]) {}');
  console.log(value);
}

run();
