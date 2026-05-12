async function add(a: number, b: number): Promise<number> {
  return a + b;
}

async function main() {
  const result = await add(5, 7);
  console.log(result);
}

main();
