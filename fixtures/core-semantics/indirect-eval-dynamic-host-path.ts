// Runtime-source indirect eval lowers to the audited host eval lane.
let source = 'let value = "indirect";';
globalThis.eval(source);
