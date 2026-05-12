function getArgs() {
    return arguments;
}

function getFirst() {
    let args = getArgs(10, 20, 30);
    return args[0];
}

function getLength() {
    let args = getArgs(1, 2, 3, 4, 5);
    return args.length;
}

console.log(getFirst());
console.log(getLength());
