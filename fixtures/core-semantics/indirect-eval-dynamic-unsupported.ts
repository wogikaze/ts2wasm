// Runtime-source indirect eval still requires the audited host eval lane.
let source = 'let value = "indirect";';
globalThis.eval(source);
