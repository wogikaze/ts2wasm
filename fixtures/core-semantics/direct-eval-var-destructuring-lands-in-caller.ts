function run() {
  let result = eval('var { value: item, nested: { leaf } } = { value: 6, nested: { leaf: 7 } }; var [first, ...rest] = [8, 9]; item + ":" + leaf + ":" + first + ":" + rest.length');
  console.log(result);
  console.log(item);
  console.log(leaf);
  console.log(first);
  console.log(rest.length);
}

run();
