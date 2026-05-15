// Nested function with arguments - inner arguments should shadow outer
// Direct nested function calls (not returned closures)

function middle() {
    // Calls inner function with its own arguments
    let inner = function() {
        return arguments[0];
    };
    return inner(42);
}

function multi() {
    function first() {
        return arguments.length;
    }
    function second() {
        return arguments[0];
    }
    let a = first(1, 2, 3);
    let b = second(99);
    return a + b;
}

function accessLength() {
    function inner() {
        return arguments.length;
    }
    return inner(10, 20, 30);
}

function accessIndex() {
    function inner() {
        return arguments[1];
    }
    return inner("a", "b", "c");
}

// Nested named function expression with arguments
function namedExpr() {
    let fn = function myName() {
        return arguments.length;
    };
    return fn(7, 8, 9);
}

console.log(middle());
console.log(multi());
console.log(accessLength());
console.log(accessIndex());
console.log(namedExpr());
