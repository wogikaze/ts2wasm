function sameValue(actual: any, expected: any) {
  console.log(actual === expected);
}

const key = 1 + 2 - 3 * 4 / 5 ** 6;
const o: any = {
  [key]: 2.999232,
};

const direct = o[key];
const stringKey = o[String(key)];

console.log(direct);
console.log(stringKey);
console.log(direct === 2.999232);
console.log(stringKey === 2.999232);
sameValue(direct, 2.999232);
sameValue(stringKey, 2.999232);
