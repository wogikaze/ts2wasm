function run() {
  let result = eval('for (var { item } = { item: 6 }; false;) {} for (var [first, ...rest] = [8, 9]; false;) {} item + ":" + first + ":" + rest.length');
  console.log(result);
  console.log(item);
  console.log(first);
  console.log(rest.length);
}

run();
