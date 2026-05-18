function f() {
  let source = "({ items: [7, 8] })";
  console.log(eval(source).items.length);
  console.log(eval(source).items[0]);
  console.log(eval(source).items[1]);
  console.log(eval(source).items[2]);
}

f();
