// Sequential async operations
async function double(x: number): Promise<number> {
  return x * 2;
}

async function main() {
  const a = await double(10);
  const b = await double(20);
  console.log(a);
  console.log(b);
}

main();
