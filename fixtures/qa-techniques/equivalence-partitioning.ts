function categorizeAge(age: number): string {
    if (age < 0) return "Invalid";
    if (age >= 0 && age <= 12) return "Child";
    if (age >= 13 && age <= 19) return "Teen";
    if (age >= 20 && age <= 64) return "Adult";
    return "Senior";
}

console.log(categorizeAge(-5));
console.log(categorizeAge(5));
console.log(categorizeAge(15));
console.log(categorizeAge(30));
console.log(categorizeAge(70));
