async function return_value(): Promise<number> {
  return 42;
}

async function main() {
  const val = await return_value();
  console.log(val);
}

main();
