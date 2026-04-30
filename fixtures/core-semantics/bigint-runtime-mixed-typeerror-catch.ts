try {
  console.log("before");
  let a = 1n;
  a + 2;
  console.log("after");
} catch (e) {
  console.log(e.message);
}
