# class-static-block-super-unsupported

```typescript
class Parent {
  static value() {
    return 1;
  }
}

class Child extends Parent {
  static {
    console.log(super.value());
  }
}

```

**Path:** `fixtures/core-semantics/class-static-block-super-unsupported.ts`
