// Comprehensive ambient declaration erasure test
// All declare forms in this file should build successfully.

// Category A: Erased declarations
declare function externalFunc(x: number): string;
export declare function exportedFunc(): void;
declare function genericFunc<T>(value: T): T;

declare const CONST_VAL: number;
declare let mutableRef: string;
declare var legacyGlobal: boolean;
export declare const EXPORTED_CONST: string;

declare class AmbientBase {
  base: number;
}
declare class AmbientDerived extends AmbientBase {
  value: number;
  read(): number;
}
export declare class ExportedAmbientClass {
  x: number;
}

declare enum AmbientNumeric {
  A,
  B = 2,
  C
}
declare enum AmbientString {
  X = "x",
  Y = "y"
}

// Category A: Namespace erasure
declare namespace MyNamespace {
  let x: number;
}
namespace NonDeclareNamespace {
  let z: number;
}

// Category A: Class element declare
class RuntimeClass {
  declare field: number;
  declare static staticField: string;
  declare readonly readonlyField: boolean;

  readField() {
    return this.field;
  }
}

// Runtime code after ambient declarations
let runtimeValue = 42;
let runtimeString = "hello";

console.log(runtimeValue);
console.log(runtimeString);

let box = new RuntimeClass();
console.log(box.readField());
