class C {
    #value: number = 0;
    set #setter(v: number) {
        this.#value = v;
    }

    static setOther(c: C, v: number) {
        c.#setter = v;
    }

    static getOtherValue(c: C): number {
        return c.#value;
    }
}

const c1 = new C();
C.setOther(c1, 42);
console.log(C.getOtherValue(c1));

const c2 = new C();
try {
    // This should fail because {} doesn't have the brand of C
    C.setOther({} as any, 100);
} catch (e) {
    console.log(e.name);
}
