async function get_a(): Promise<number> {
  return 10;
}

async function get_b(): Promise<number> {
  return 20;
}

async function main() {
  const a = await get_a();
  const b = await get_b();
  console.log(a + b);
}

main();
