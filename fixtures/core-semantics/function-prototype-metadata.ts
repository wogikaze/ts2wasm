function score(left: number, right: number, bonus: number): number {
    return left + right + bonus;
}

function empty(): number {
    return 1;
}

function greet(name: string): string {
    return "Hello, " + name;
}

console.log(score.name);
console.log(score.length);
console.log(score.toString());
console.log(empty.name);
console.log(empty.length);
console.log(empty.toString());
console.log(greet.name);
console.log(greet.length);
