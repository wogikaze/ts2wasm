let sparse = [1, , 3];

function observe(a, b, c) {
  console.log(a);
  console.log(b === undefined);
  console.log(c);
}

observe(...sparse);
