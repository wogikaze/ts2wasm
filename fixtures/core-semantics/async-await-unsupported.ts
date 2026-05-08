// Async function syntax — should produce unsupported diagnostic
async function fetchData(): Promise<string> {
  return "data";
}
const result = await fetchData();
console.log(result);
