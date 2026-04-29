# class-super-method

```typescript
class Base {
    value() {
        return 4;
    }
}

class Child extends Base {
    value() {
        return super.value();
    }
}

let c = new Child();
console.log(c.value());

```

**Path:** `fixtures/classes-and-inheritance/class-super-method.ts`
