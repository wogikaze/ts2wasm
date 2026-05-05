function keysForArgs(x, y, z) {
    let keys = Object.keys(arguments);
    console.log(keys.length);
    console.log(keys[0]);
    console.log(keys[1]);
    console.log(keys[2]);
    console.log(keys.length === 3);
}

keysForArgs(1, 2, 3);
