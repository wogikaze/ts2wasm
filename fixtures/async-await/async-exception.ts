async function fail(): Promise<number> {
  throw new Error("async failure");
}

async function main() {
  try {
    await fail();
  } catch (e) {
    console.log(e.message);
  }
}

main();
