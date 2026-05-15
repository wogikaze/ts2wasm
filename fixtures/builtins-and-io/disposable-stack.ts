// DisposableStack basic construction — runtime may be stub/unsupported
const stack = new DisposableStack();
console.log(typeof stack);
console.log(typeof stack.dispose);
console.log(typeof stack.use);
console.log(typeof stack.adopt);
console.log(typeof stack.defer);
