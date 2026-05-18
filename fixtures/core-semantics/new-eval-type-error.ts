try {
  new eval("1 + 1");
  console.log("unreachable");
} catch (error) {
  console.log(error.name);
}
