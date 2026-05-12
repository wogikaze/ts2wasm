function first() {
    return arguments[0];
}

function second() {
    return arguments[1];
}

function sumToN(n) {
    let total = 0;
    let i = 0;
    while (i < arguments.length) {
        total = total + arguments[i];
        i = i + 1;
    }
    return total;
}

console.log(first(10));
console.log(first(42));
console.log(second(1, 2));
console.log(second(10, 20));
console.log(sumToN(0));
console.log(sumToN(5, 10));
console.log(sumToN(5, 10, 15));
