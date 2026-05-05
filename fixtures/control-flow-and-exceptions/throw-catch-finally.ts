try {
  throw "caught";
} catch (e) {
  console.log(e);
} finally {
  console.log("finally");
}
