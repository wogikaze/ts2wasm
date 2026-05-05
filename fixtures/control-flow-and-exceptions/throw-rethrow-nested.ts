try {
  try {
    throw "first";
  } catch (e) {
    console.log(e);
    throw "second";
  } finally {
    console.log("inner finally");
  }
} catch (e) {
  console.log(e);
} finally {
  console.log("outer finally");
}
