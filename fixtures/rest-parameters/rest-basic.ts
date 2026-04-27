// Basic rest parameter test
// Verifies that rest parameters collect remaining arguments.

function sum(...args) {
    return args.length;
}

console.log(sum(1, 2, 3));
