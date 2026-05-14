const first: any = Symbol("slot");
const second: any = Symbol("slot");

function ID(value: any): any {
  return value;
}

const obj: any = {
  a() {
    return "A";
  },
  [first]() {
    return this.prefix + " first";
  },
  c() {
    return "C";
  },
  [ID(second)]() {
    return "symbol second";
  },
  prefix: "symbol",
};

console.log(obj.a());
console.log(obj[first]());
console.log(obj.c());
console.log(obj[second]());
console.log(first === second ? "same" : "different");
