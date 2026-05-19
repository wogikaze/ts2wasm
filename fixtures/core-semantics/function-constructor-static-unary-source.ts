let negativeBody = Function(-1);
let positiveBody = new Function(+1);
let voidBody = Function(void 0);
let notBody = Function(!0);

console.log(negativeBody());
console.log(positiveBody());
console.log(negativeBody.toString());
console.log(voidBody());
console.log(voidBody.toString());
console.log(notBody());
console.log(notBody.toString());
