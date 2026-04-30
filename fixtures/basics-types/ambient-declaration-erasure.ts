declare class AmbientBase {
  base: number;
}

declare class AmbientDerived extends AmbientBase {
  value: number;
  read(): number;
}

declare function readAmbient(value: AmbientDerived): number;
declare const ambientName: string;
declare enum AmbientEnum {
  A,
  B = 2
}

class RuntimeBox {
  declare prop: string;

  read() {
    return 1;
  }
}

let box = new RuntimeBox();
console.log(box.read());
