const object: any = {
  [1.1]: "fraction",
  [1.e1]: "ten",
  [1.10e1]: "eleven",
  [1.23e1]: "shifted",
};

console.log(object[1.1]);
console.log(object[String(1.1)]);
console.log(object[10]);
console.log(object[String(1.e1)]);
console.log(object[String(1.10e1)]);
console.log(object[String(1.23e1)]);
