function testcase() {
  let desc = Object.getOwnPropertyDescriptor(arguments, "callee");
  console.log(desc);
}

testcase();
