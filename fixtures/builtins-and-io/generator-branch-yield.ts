function* thenGen() {
  let chooseThen = true;
  if (chooseThen) {
    yield "then";
  } else {
    yield "else";
  }
}

function* elseGen() {
  let chooseThen = false;
  if (chooseThen) {
    yield "then";
  } else {
    yield "else";
  }
}

let result = thenGen().next();
console.log(result.value);
console.log(result.done);

result = elseGen().next();
console.log(result.value);
console.log(result.done);
