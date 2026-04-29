# static-combined-named-import-unsupported

```typescript
// Diagnostic fixture for missing local combined default/named imports tracked by issue 232.
import value, { named as renamed } from "./module-source";
console.log(value, renamed);

```

**Path:** `fixtures/module-system/static-combined-named-import-unsupported.ts`
