function sameValue(actual, expected) {
    console.log(arguments.length);
    console.log(actual === expected);
}

let assert = { sameValue: sameValue };

assert.sameValue(1, 1);
