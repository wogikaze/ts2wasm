declare namespace MyNS {
  let x: number;
}
class C {
  declare get accessor(): number;
}
console.log("ambient namespace/module erasure ok");
