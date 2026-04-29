let { outer: { value } } = { outer: { value: 1 } };
console.log(value);

let { outer: { left, right: renamed } } = { outer: { left: 2, right: 3 } };
console.log(left);
console.log(renamed);

function pick({ outer: { value } }) {
  return value;
}

let pickAlias = ({ outer: { value: renamed } }) => renamed;

console.log(pick({ outer: { value: 4 } }));
console.log(pickAlias({ outer: { value: 5 } }));
