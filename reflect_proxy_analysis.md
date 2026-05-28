# Reflect and Proxy API Analysis

## Summary of Findings

### Reflect API

#### What's Implemented:
1. **RuntimeFn enum** (runtime-catalog/src/runtime_fn.rs): All 12 Reflect methods are defined
   - ReflectDefineProperty, ReflectDeleteProperty, ReflectGet, ReflectHas
   - ReflectOwnKeys, ReflectPreventExtensions, ReflectSet, ReflectSetPrototypeOf
   - ReflectApply, ReflectConstruct

2. **WAT implementations** (backend-wasm/src/runtime/object/reflect.rs): All 12 methods implemented
   - 10 methods have full WAT implementations (defineProperty, deleteProperty, get, has, ownKeys, preventExtensions, set, setPrototypeOf)
   - ReflectApply and ReflectConstruct delegate to host imports ($host_reflect_apply, $host_reflect_construct)

3. **IR routing** (ir/src/lowered/program_builtins.rs): All Reflect methods properly routed

4. **Dispatch** (backend-wasm/src/runtime_dispatch_object.rs): All 12 methods properly dispatched

#### Issue: Reflect constructor
**backend-wasm/src/runtime_dispatch_object.rs:120-128**: Reflect constructor returns error:
```
if class_name == "Reflect" {
    return Err(Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: "issue-106: Reflect API is not implemented yet".to_owned(),
        span: Some(span),
        phase: None,
    });
}
```

**Status**: Reflect constructor not implemented yet. Only Reflect.* method calls work.

---

### Proxy API

#### What's Implemented:
1. **ProxyTrapKind enum** (ir/src/lowered/facts/proxy.rs): All 12 proxy traps defined
   - ProxyGet, ProxySet, ProxyHas, ProxyDeleteProperty, ProxyConstruct, ProxyApply
   - ProxyGetPrototypeOf, ProxySetPrototypeOf, ProxyIsExtensible, ProxyPreventExtensions
   - ProxyGetOwnPropertyDescriptor, ProxyDefineProperty, ProxyOwnKeys

2. **IR routing** (ir/src/lowered/resolver/call/method.rs:2587-2599): Proxy dispatch implemented
   - Compile-time proxy dispatch for Reflect.* and Object.* methods
   - Proxy callable detection (ir/src/lowered/resolver/call/user.rs:282-290)

3. **Proxy binding detection** (ir/src/lowered/resolver/expr/facts.rs:281-291):
   ```
   pub(crate) fn resolved_expr_proxy_binding(
       ctx: &LoweringCtx,
       expr: &ResolvedExpr,
   ) -> Option<ProxyBinding> {
       let ResolvedExpr::Ident(name) = expr else {
           return None;
       };
       ctx.resolve_local(name)
           .ok()
           .and_then(|local_id| ctx.facts.proxy_locals.get(&local_id).cloned())
   }
   ```

#### Issue: Proxy constructor
**ir/src/lowered/resolver/call/constructor.rs:108-119**: Proxy constructor wrongly returns target:
```
if class_name == "Proxy" {
    let [target, _handler] = args else {
        return Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: "issue-106: Proxy constructor requires target and handler arguments"
                .to_owned(),
            span: Some(span),
            phase: None,
        });
    };
    return self.lower_expr(target);  // BUG: Returns target, not proxy!
}
```

**Status**: Proxy constructor is broken. It returns the target expression instead of creating a proxy object.

**Missing Implementation**: No WAT function exists to create a Proxy object. The constructor needs to:
1. Create a Proxy object (currently just returns target)
2. Store the handler (currently ignored)
3. Emit proper WAT with $host_proxy_new function

---

## Files to Modify

### For Proxy:
1. **ir/src/lowered/resolver/call/constructor.rs:108-119**: Fix to actually create proxy object
2. **backend-wasm/src/runtime/object/**: Add WAT implementation for proxy object creation
3. **backend-wasm/src/runtime_fn_impl.rs**: Add RuntimeFn variants for proxy creation
4. **runtime-catalog/src/runtime_fn.rs**: Add Proxy construction capability

### For Reflect:
1. **ir/src/lowered/resolver/call/constructor.rs**: Remove Reflect constructor error (or implement Reflect)
2. **backend-wasm/src/runtime/**: Add WAT implementations for Reflect object creation
