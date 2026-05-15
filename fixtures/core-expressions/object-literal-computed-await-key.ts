async function run() {
  const object: any = {
    [await 9]: "awaited",
  };

  console.log(object[await 9]);
  console.log(object[String(await 9)]);
}

run();
