# class-expression

```typescript
const Pair = class {
    constructor(left, right) {
        this.left = left;
        this.right = right;
    }

    total() {
        return this.left + this.right;
    }
};

let pair = new Pair(5, 6);
console.log(pair.total());

```

**Path:** `fixtures/classes-and-inheritance/class-expression.ts`
