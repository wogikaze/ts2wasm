// if/else branching with truthiness
let x = 10;
if (x > 5) {
  console.log("greater");
} else {
  console.log("less or equal");
}

if (x > 20) {
  console.log("very large");
} else if (x > 5) {
  console.log("moderate");
} else {
  console.log("small");
}

// truthiness: 0 is falsy, non-zero is truthy
if (0) {
  console.log("zero is truthy");
} else {
  console.log("zero is falsy");
}

// empty string is falsy
if ("") {
  console.log("empty string is truthy");
} else {
  console.log("empty string is falsy");
}

// null is falsy
if (null) {
  console.log("null is truthy");
} else {
  console.log("null is falsy");
}

// undefined is falsy
if (undefined) {
  console.log("undefined is truthy");
} else {
  console.log("undefined is falsy");
}
