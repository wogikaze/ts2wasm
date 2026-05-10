// Dynamic import expression
async function main() {
  const mod = await import("./dynamic-import-helper.ts");
  console.log(mod.hello);
}
main();
