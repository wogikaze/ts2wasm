function isConstructor(f: any): boolean {
  try {
    Reflect.construct(function () {}, [], f);
  } catch (e) {
    return false;
  }
  return true;
}

console.log(String(isConstructor(Error)));
console.log(String(isConstructor(AggregateError)));
console.log(String(isConstructor(Boolean)));
console.log(String(isConstructor(Symbol)));
console.log(String(isConstructor(Error.prototype.toString)));
