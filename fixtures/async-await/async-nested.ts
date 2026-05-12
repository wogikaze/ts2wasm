async function add(a: number, b: number): Promise<number> {
  return a + b;
}

async function multiplySum(x: number, y: number, z: number): Promise<number> {
  const sum = await add(x, y);
  return sum * z;
}

async function main() {
  const result = await multiplySum(2, 3, 4);
  console.log(result);
}

main();
