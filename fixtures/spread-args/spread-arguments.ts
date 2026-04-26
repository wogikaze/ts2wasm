// Test spread arguments in function calls

function add(a, b, c) {
    return a + b + c;
}

let x = add(...[1, 2, 3]);
