// Basic rest parameter test
// Note: Full rest parameter implementation requires collecting remaining arguments
// This fixture tests that rest parameters parse and compile without errors

function sum(...args) {
    return args.length;
}

console.log(sum(1, 2, 3));
