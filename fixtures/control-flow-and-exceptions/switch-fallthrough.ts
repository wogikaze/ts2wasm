let value = 2;
let fallthrough = 0;
switch (value) {
  case 1:
    console.log("one");
  case 2:
    console.log("two");
    fallthrough = fallthrough + 10;
  case 3:
    console.log("three");
    fallthrough = fallthrough + 1;
    break;
  default:
    console.log("bad-default-after-break");
}
console.log(fallthrough);

let missing = 9;
let defaultOrder = 0;
switch (missing) {
  case 1:
    console.log("bad-before-default");
    break;
  default:
    console.log("default-middle");
    defaultOrder = defaultOrder + 1;
  case 2:
    console.log("after-default");
    defaultOrder = defaultOrder + 2;
    break;
  case 3:
    console.log("bad-after-break");
}
console.log(defaultOrder);

let matchedAfterDefault = 0;
switch (2) {
  case 1:
    matchedAfterDefault = matchedAfterDefault + 1000;
    break;
  default:
    matchedAfterDefault = matchedAfterDefault + 100;
  case 2:
    matchedAfterDefault = matchedAfterDefault + 2;
    break;
}
console.log(matchedAfterDefault);

let explicitBreak = 0;
switch (1) {
  case 1:
    explicitBreak = explicitBreak + 1;
    break;
  case 2:
    explicitBreak = explicitBreak + 100;
  default:
    explicitBreak = explicitBreak + 1000;
}
console.log(explicitBreak);
