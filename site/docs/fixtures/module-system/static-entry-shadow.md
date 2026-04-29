# static-entry-shadow

```typescript
// Fixture for issue 233 source-backed static named import binding lowering.
import { value as importedValue } from "./static-entry-source";
const value = 99;
console.log(importedValue);

```

**Path:** `fixtures/module-system/static-entry-shadow.ts`
