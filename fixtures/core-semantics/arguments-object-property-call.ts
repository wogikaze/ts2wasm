function sameValue(actual, expected) {
    console.log(arguments.length);
    console.log(actual === expected);
}

let checks = { sameValue: sameValue };

checks.sameValue(1, 1);
