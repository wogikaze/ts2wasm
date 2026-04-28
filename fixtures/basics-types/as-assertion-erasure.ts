let value = 3 as number;
let nested = ({ x: value } as { x: number });
let chained = [value] as number[] as unknown;

console.log(value);
console.log(nested.x);
console.log(chained[0]);
