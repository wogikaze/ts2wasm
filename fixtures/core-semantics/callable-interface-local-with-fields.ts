interface I1<T> {
    (value: T): void;
    field1: I1<boolean>;
}
function foo() {
    var test!: I1<string>;
    test("expects boolean instead of string");
    test(true);
}
