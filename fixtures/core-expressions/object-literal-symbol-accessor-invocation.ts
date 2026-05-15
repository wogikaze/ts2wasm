const first: any = Symbol("slot");
const second: any = Symbol("slot");

const obj: any = {
  get [first]() {
    return this.value;
  },
  set [first](value) {
    this.value = value;
  },
  get [second]() {
    return "second";
  },
};

obj[first] = "first";
console.log(obj[first]);
console.log(obj[second]);
console.log(first === second ? "same" : "different");
