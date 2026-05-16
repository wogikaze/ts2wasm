// Reflect.construct is now supported via host shim.
// See reflect-apply-construct.ts for the actual implementation test.
const target = { x: 42 };
console.log(Reflect.construct(target, []));
