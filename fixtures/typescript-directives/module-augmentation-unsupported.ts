// Module augmentation — should produce unsupported diagnostic
declare module "some-library" {
  export interface ExtraAPI {
    doSomething(): void;
  }
}
console.log("module augmentation");
