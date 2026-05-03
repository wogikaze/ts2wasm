declare namespace MyNS {
  let x: number;
}
declare module "external" {
  export let y: number;
}
class C {
  declare get accessor(): number;
}
console.log("ambient namespace/module erasure ok");
