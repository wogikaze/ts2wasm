function rhs(value) {
  console.log("rhs");
  return value;
}

let andSkip = 0;
console.log(andSkip &&= rhs(1));
console.log(andSkip);

let andRun = 3;
console.log(andRun &&= rhs(4));
console.log(andRun);

let orSkip = "left";
console.log(orSkip ||= rhs("bad"));
console.log(orSkip);

let orRun = false;
console.log(orRun ||= rhs(2));
console.log(orRun);

let nullishSkip = 7;
console.log(nullishSkip ??= rhs(8));
console.log(nullishSkip);

let nullishRun = null;
console.log(nullishRun ??= rhs(5));
console.log(nullishRun);
