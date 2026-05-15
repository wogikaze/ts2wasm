// Async function with try/catch (simple success path)
async function safeSuccess(value: number): Promise<number> {
  return value * 2;
}

async function main() {
  try {
    const r = await safeSuccess(21);
    console.log(r);
  } catch (e) {
    console.log("should not reach");
  }
}

main();
