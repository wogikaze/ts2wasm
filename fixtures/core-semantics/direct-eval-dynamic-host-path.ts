// Runtime-source direct eval lowers to the audited host eval lane.
let x = "1 + 1";
eval(x);
console.log("unreachable");
