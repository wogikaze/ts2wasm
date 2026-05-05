let sideEffectCount = 0;

function sideEffect(): number {
    sideEffectCount++;
    return 42;
}

// void expr evaluates for side effects and returns undefined
const result = void sideEffect();
console.log("sideEffectCount:", sideEffectCount);

// Verify result is undefined
console.log("result === undefined:", result === undefined);
console.log("typeof result:", typeof result);
