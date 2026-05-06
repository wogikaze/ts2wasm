// Only a class declaration at top level with a method.
// Tests that the emit_top_level_statements $exception_pending guard selects
// the corresponding runtime globals even when the class is the only
// top-level statement (no let/expr/return to incidentally select globals).
class Foo {
    bar() {
        return 42;
    }
}
