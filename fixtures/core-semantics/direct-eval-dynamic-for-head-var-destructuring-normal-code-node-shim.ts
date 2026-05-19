function run() {
  let source = "for (var {item} of [{item: 6}]) {} for (var [first, ...rest] of [[8, 9]]) {}";
  eval(source);
  console.log(item);
  console.log(first);
  console.log(rest[0]);
}

run();
