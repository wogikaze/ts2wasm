try {
  console.log("before");
  let a = 6n;
  let z = 0n;
  a / z;
  console.log("after");
} catch (e) {
  console.log(e.message);
}
