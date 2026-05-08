// Nested namespace/module resolution: A.B.C
namespace A {
  export namespace B {
    export namespace C {
      export const value = 42;
    }
  }
}

console.log(A.B.C.value);
