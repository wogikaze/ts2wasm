function makeReader() {
  let value = "escaped-closure";

  function read() {
    return value;
  }

  return read;
}

let reader = makeReader();
console.log(reader());
