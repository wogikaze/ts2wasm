// nested if/else
let a = 1;
let b = 2;
let c = 3;

if (a === 1) {
  if (b === 2) {
    if (c === 3) {
      console.log("all match");
    } else {
      console.log("c mismatch");
    }
  } else {
    console.log("b mismatch");
  }
} else {
  console.log("a mismatch");
}

// nested with else-if chains inside branches
let score = 85;
if (score >= 90) {
  console.log("A");
} else {
  if (score >= 80) {
    console.log("B");
  } else {
    if (score >= 70) {
      console.log("C");
    } else {
      console.log("F");
    }
  }
}
