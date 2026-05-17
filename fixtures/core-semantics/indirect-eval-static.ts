// Static indirect eval literals are compiled through the AOT eval lane.
let viaComma = (0, eval)("1 + 2");
let viaMember = globalThis.eval('"member"');
let viaIndex = globalThis["eval"]("3 + 4");

console.log(viaComma);
console.log(viaMember);
console.log(viaIndex);
