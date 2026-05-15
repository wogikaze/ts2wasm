// Multiple concurrent async operations
async function delay(ms: number, value: number): Promise<number> {
  return value;
}

async function main() {
  const [a, b] = await Promise.all([delay(1, 10), delay(1, 20)]);
  console.log(a);
  console.log(b);
}

main();
