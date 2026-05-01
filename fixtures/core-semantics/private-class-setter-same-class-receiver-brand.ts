class C {
    #value = 0;
    set #x(v: number) { this.#value = v; }

    static setOther(c: C, v: number) {
        c.#x = v;
    }

    getValue() { return this.#value; }
}

const c1 = new C();
const c2 = new C();
C.setOther(c1, 42);
console.log(c1.getValue()); // 42

const other = {};
try {
    C.setOther(other as any, 99);
} catch (e) {
    console.log((e as Error).name); // TypeError
}
