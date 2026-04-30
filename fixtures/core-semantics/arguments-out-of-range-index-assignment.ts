function writeBeyondFirstArgument(first) {
    console.log(arguments.length);
    console.log(arguments[0]);
    arguments[7] = 12;
    console.log(arguments[7]);
    console.log(arguments.length);
    console.log(arguments[1]);
    console.log(first);
}

writeBeyondFirstArgument(30);
