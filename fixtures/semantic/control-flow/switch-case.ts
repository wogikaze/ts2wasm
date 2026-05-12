// basic switch with break
let x = 2;
switch (x) {
  case 1:
    console.log("one");
    break;
  case 2:
    console.log("two");
    break;
  case 3:
    console.log("three");
    break;
  default:
    console.log("other");
    break;
}

// switch with default only (no matching case)
let y = 99;
switch (y) {
  case 1:
    console.log("uno");
    break;
  default:
    console.log("default case");
    break;
}

// switch with string cases
let s = "hello";
switch (s) {
  case "world":
    console.log("world");
    break;
  case "hello":
    console.log("greeting");
    break;
  default:
    console.log("unknown");
    break;
}
