// Chained async operations
async function step1(n: number): Promise<number> {
  return n + 1;
}

async function step2(n: number): Promise<number> {
  return n * 2;
}

async function step3(n: number): Promise<number> {
  return n - 3;
}

async function main() {
  const result = await step3(await step2(await step1(5)));
  console.log(result);
}

main();
