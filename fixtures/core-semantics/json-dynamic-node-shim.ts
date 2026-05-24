const makeSource: any = Function("return '{\"n\":1,\"label\":\"ok\"}'");
const source: any = makeSource();
const parsed: any = JSON.parse(source);
console.log(parsed.n);
console.log(parsed.label);
console.log(JSON.stringify({ n: parsed.n, label: parsed.label }));
