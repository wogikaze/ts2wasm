<<<<<<< Updated upstream
// Dynamic function.call/apply/bind on known function declarations
function add(a: number, b: number) {
    return a + b;
}

function addBase(this: any, value: number) {
    return this.base + value;
}

const ctx = { base: 10 };

// Static function.call with function name - should already work
console.log(add.call(undefined, 1, 2));

// Dynamic: function through a local variable
var myFunc = add;
console.log(myFunc.call(undefined, 3, 4));

// Function.apply with array
console.log(add.apply(undefined, [5, 6]));

// Function.bind with partial args, then call
var bound = add.bind(undefined, 7);
console.log(bound(8));

// Function.bind with this context
var boundBase = addBase.bind(ctx, 5);
console.log(boundBase());
||||||| Stash base
=======
const obj = { foo: 42 };
const hasOwn = Object.prototype.hasOwnProperty.call(obj, "foo");
console.log(hasOwn);
console.log(Object.prototype.hasOwnProperty.call(obj, "bar"));
>>>>>>> Stashed changes
