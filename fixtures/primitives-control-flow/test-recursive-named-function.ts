const __func = function __exp__func(arg: number): number {
    if (arg === 1) {
        return arg;
    } else {
        return __exp__func(arg - 1) * arg;
    }
};

const fact_of_3 = __func(3);
console.log(fact_of_3);
