// TypeScript module augmentation — should produce unsupported diagnostic
declare module "my-module" {
  export const foo: number;
}
let x = 1;
console.log(x);
