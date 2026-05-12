async function doubleValue(n: number): Promise<number> {
  return n * 2;
}

async function main() {
  const val = await doubleValue(21);
  console.log(val);
}

main();
