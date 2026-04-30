function join(a: string, b: string, c: string) {
  console.log(a + b + c);
}

let letters = new Set(["a", "b", "c"]);
join(...letters);
