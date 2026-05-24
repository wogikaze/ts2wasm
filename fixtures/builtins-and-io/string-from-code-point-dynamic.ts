function make(code) {
  let value = String.fromCodePoint(code);
  console.log(value);
}

make(65);
make(0x100);
make(0x1f44b);
