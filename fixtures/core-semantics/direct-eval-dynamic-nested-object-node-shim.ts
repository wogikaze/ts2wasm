function f() {
  let source = "({ child: { value: 7, label: 'ok' } })";
  console.log(eval(source).child.value);
  console.log(eval(source).child.label);
  console.log(eval(source).child.missing);
}

f();
