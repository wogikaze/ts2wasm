// @ts-nocheck
// instanceof operator with class inheritance
class Animal {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
}

class Dog extends Animal {
  breed: string;
  constructor(name: string, breed: string) {
    super(name);
    this.breed = breed;
  }
}

const a = new Animal("generic");
const d = new Dog("Rex", "Husky");

console.log(d instanceof Dog);
console.log(d instanceof Animal);
console.log(a instanceof Animal);
console.log(a instanceof Dog);
console.log(a.name);
console.log(d.name);
console.log(d.breed);
