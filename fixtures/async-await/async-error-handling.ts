// Async error handling with try/catch
async function mightFail(shouldFail: boolean): Promise<string> {
  if (shouldFail) {
    throw new Error("failed");
  }
  return "ok";
}

async function main() {
  try {
    const r1 = await mightFail(false);
    console.log(r1);
    const r2 = await mightFail(true);
    console.log(r2);
  } catch (e) {
    console.log("caught");
  }
}

main();
