// Annex B block-level function hoisting
if (true) {
  function hoisted() {
    console.log("hoisted");
  }
}
hoisted();
