function sideEffect(): number {
    console.log("sideEffectCallCount: 1");
    return 42;
}

// void expr evaluates for side effects and returns undefined
const result = void sideEffect();
console.log("typeof result:", typeof result);
console.log("result === undefined:", result === undefined);
