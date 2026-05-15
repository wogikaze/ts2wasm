// Class instanceof checks
class Animal {}
class Dog extends Animal {}
class Cat extends Animal {}

const d = new Dog();
const c = new Cat();
console.log(d instanceof Dog);
console.log(d instanceof Animal);
console.log(d instanceof Cat);
console.log(c instanceof Animal);
console.log(c instanceof Dog);
