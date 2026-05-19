function run() {
  let result = eval("for (var { item } of [{ item: 6 }]) {} for (var [first, ...rest] of [[8, 9]]) {} item + ':' + first + ':' + rest.length");
  console.log(result);
  console.log(item);
  console.log(first);
  console.log(rest.length);
}

run();
