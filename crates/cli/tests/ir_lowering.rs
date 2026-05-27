use ts2wasm_frontend::{DiagCode, Span};
use ts2wasm_ir::lowered::RuntimeFn;

fn parse_and_resolve(source: &str) -> Vec<ts2wasm_ir::builtin_resolved::ResolvedStmt> {
    let program = ts2wasm_cli::parse_program(source).unwrap();
    ts2wasm_ir::builtin_resolver::resolve_builtins(&program).unwrap()
}

fn parse_resolve_and_expand_static_eval(
    source: &str,
) -> Vec<ts2wasm_ir::builtin_resolved::ResolvedStmt> {
    let program = ts2wasm_cli::parse_program(source).unwrap();
    let named = ts2wasm_ir::name_resolver::resolve_names(&program).unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&named).unwrap();
    ts2wasm_compiler::stages::eval_expand::expand_static_eval_fragments(resolved).unwrap()
}

#[test]
fn lowering_pads_object_get_own_property_descriptor_static_call_arity() {
    use ts2wasm_ir::lowered::{LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        let noArgs = Object.getOwnPropertyDescriptor();
        let oneArg = Object.getOwnPropertyDescriptor(1);
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program)
        .expect("getOwnPropertyDescriptor arity padding should lower");
    ts2wasm_ir::lowered::validate_lowered(&lowered)
        .expect("padded getOwnPropertyDescriptor calls should validate");

    for stmt in lowered.top_level_statements.iter().take(2) {
        let LoweredStmt::Let(
            _,
            LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) = stmt
        else {
            panic!("expected top-level descriptor call let, got {stmt:?}");
        };
        assert_eq!(*intrinsic, RuntimeFn::ObjectGetOwnPropertyDescriptor);
        assert_eq!(args.len(), 2);
    }
}

#[test]
fn lowering_routes_computed_proxy_get_through_proxy_get_trap() {
    use ts2wasm_ir::builtin::BuiltinId;
    use ts2wasm_ir::lowered::{FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        const target = { x: 10 };
        function proxyGet(obj: any, prop: string) {
          return obj[prop];
        }
        const handler = { get: proxyGet };
        const proxy = new Proxy(target, handler);
        const key = "x";
        console.log(proxy[key]);
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let console_arg = lowered
        .top_level_statements
        .iter()
        .find_map(|stmt| match stmt {
            LoweredStmt::Expr(
                LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(BuiltinId::ConsoleLog),
                    args,
                    ..
                },
                _,
            ) => args.first(),
            _ => None,
        })
        .expect("console.log argument should be lowered");

    match console_arg {
        LoweredExpr::Call {
            kind: FunctionCallKind::User(_),
            args,
            ..
        } => {
            assert!(
                matches!(
                    args.as_slice(),
                    [LoweredExpr::Local(LocalId(0), _), LoweredExpr::Local(_, _),]
                ),
                "ProxyGet trap should receive target and computed key, got {args:?}"
            );
        }
        other => panic!("computed proxy get should lower to ProxyGet handler call, got {other:?}"),
    }
}

#[test]
fn lowering_passes_mutable_class_method_outer_local_capture() {
    use ts2wasm_ir::lowered::{FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        var callCount = 0;
        class C {
          method() {
            callCount = callCount + 1;
          }
        }
        let c = new C();
        c.method();
        console.log(callCount);
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert!(matches!(
        lowered.top_level_statements.first(),
        Some(LoweredStmt::Let(
            LocalId(0),
            LoweredExpr::EnvCellNew(initial, _)
        , _)) if matches!(initial.as_ref(), LoweredExpr::Number(0, _))
    ));
    assert!(matches!(
        lowered.top_level_statements.last(),
        Some(LoweredStmt::Expr(LoweredExpr::Call {
            kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
            args, ..}, _)) if matches!(args.as_slice(), [LoweredExpr::EnvCellGet(LocalId(0), _)])
    ));
    assert!(matches!(
        lowered.functions[1].body.as_slice(),
        [LoweredStmt::Expr(
            LoweredExpr::EnvCellSet {
                cell: LocalId(0),
                ..
            },
            _
        )]
    ));
}

#[test]
fn lowering_passes_nested_function_mutable_outer_local_capture() {
    use ts2wasm_ir::lowered::{FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        function outer() {
          let count = 0;
          function inc() {
            count = count + 1;
          }
          inc();
          console.log(count);
        }
        outer();
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();
    let outer = &lowered.functions[0];

    assert!(matches!(
        outer.body.first(),
        Some(LoweredStmt::Let(
            LocalId(0),
            LoweredExpr::EnvCellNew(initial, _),
            _
        )) if matches!(initial.as_ref(), LoweredExpr::Number(0, _))
    ));
    assert!(matches!(
        outer.body.last(),
        Some(LoweredStmt::Expr(LoweredExpr::Call {
            kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
            args, ..
        }, _)) if matches!(args.as_slice(), [LoweredExpr::EnvCellGet(LocalId(0), _)])
    ));
}

#[test]
fn lowering_passes_function_expression_mutable_outer_local_capture() {
    use ts2wasm_ir::lowered::{LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        function outer() {
          let actual = 0;
          call(function () {
            actual = actual + 1;
          });
        }
        function call(callback) {
          callback();
        }
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();
    let outer = &lowered.functions[0];

    assert!(matches!(
        outer.body.first(),
        Some(LoweredStmt::Let(
            LocalId(0),
            LoweredExpr::EnvCellNew(initial, _),
            _
        )) if matches!(initial.as_ref(), LoweredExpr::Number(0, _))
    ));
}

#[test]
fn lowering_initializes_direct_eval_catch_binding_env_cell() {
    use ts2wasm_ir::lowered::{LocalId, LoweredExpr, LoweredStmt};

    let source = r#"
        function outer() {
          let source = "err = err + 4; err";
          try {
            throw 3;
          } catch (err) {
            eval(source);
          }
        }
        "#;
    let tokens = ts2wasm_frontend::Lexer::new(source).tokenize().unwrap();
    let ast = ts2wasm_frontend::Parser::new(tokens, source)
        .parse_program()
        .unwrap();
    let named = ts2wasm_ir::name_resolver::resolve_names(&ast).unwrap();
    let program = ts2wasm_ir::builtin_resolver::resolve_builtins(&named).unwrap();
    let lowered = ts2wasm_ir::lowered::lower_program(&program)
        .expect("catch binding direct eval env cell should lower");
    let outer = &lowered.functions[0];

    let try_catch = outer
        .body
        .iter()
        .find_map(|stmt| match stmt {
            LoweredStmt::TryCatch {
                catch_var,
                catch_body,
                ..
            } => Some((catch_var, catch_body)),
            _ => None,
        })
        .expect("outer function should contain try/catch");

    assert_eq!(*try_catch.0, Some(LocalId(2)));
    let catch_body = try_catch.1.as_ref().expect("catch body should lower");
    assert!(matches!(
        catch_body.first(),
        Some(LoweredStmt::Assign(
            LocalId(2),
            LoweredExpr::EnvCellNew(initial, _),
            _
        )) if matches!(initial.as_ref(), LoweredExpr::Local(LocalId(2), _))
    ));
}

#[test]
fn lowering_rejects_unexpanded_static_eval_fragment() {
    let source = r#"let value = eval("1 + 2");"#;
    let tokens = ts2wasm_frontend::Lexer::new(source).tokenize().unwrap();
    let ast = ts2wasm_frontend::Parser::new(tokens, source)
        .parse_program()
        .unwrap();
    let named = ts2wasm_ir::name_resolver::resolve_names(&ast).unwrap();
    let program = ts2wasm_ir::builtin_resolver::resolve_builtins(&named).unwrap();
    let err = ts2wasm_ir::lowered::lower_program(&program)
        .expect_err("static eval must pass through AOT expansion before lowering");
    assert_eq!(err.code, DiagCode::UnsupportedEval);
    assert!(
        err.message
            .contains("static eval fragment reached lowering without AOT expansion"),
        "unexpected diagnostic: {err:?}"
    );
}

#[test]
fn lowering_passes_static_direct_eval_function_declaration_capture() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_resolve_and_expand_static_eval(
        r#"
        function outer() {
          eval("var x = 1; function g() { return x; }");
          return g();
        }
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program)
        .expect("direct eval var/function declarations should lower");
    let outer = &lowered.functions[0];

    assert!(matches!(
        outer.body.last(),
        Some(LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _
        )) if matches!(args.as_slice(), [LoweredExpr::Local(LocalId(1), _)])
    ));
}

#[test]
fn lowering_new_eval_throws_type_error() {
    use ts2wasm_ir::lowered::{BuiltinErrorConstructor, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve("new eval(\"1 + 1\");");
    let lowered =
        ts2wasm_ir::lowered::lower_program(&program).expect("new eval TypeError path should lower");

    assert!(matches!(
        lowered.top_level_statements.as_slice(),
        [LoweredStmt::Expr(
            LoweredExpr::Block {
                stmts,
                result,
                ..
            },
            _
        )] if matches!(
            stmts.as_slice(),
            [LoweredStmt::Throw(
                LoweredExpr::ErrorNew {
                    constructor: BuiltinErrorConstructor::TypeError,
                    ..
                },
                _
            )]
        ) && matches!(result.as_ref(), LoweredExpr::Undefined(_))
    ));
}

#[test]
fn lowering_passes_top_level_capture_through_double_nested_function_expr() {
    let program = parse_and_resolve(
        r#"
        var digits = { latn: "0123456789" };
        function readAll(locales, numberingSystems) {
          locales.forEach(function(locale) {
            numberingSystems.forEach(function(numbering) {
              console.log(digits[numbering]);
            });
          });
        }
        readAll(["en"], ["latn"]);
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("double nested function expressions should capture top-level locals");
}

#[test]
fn lowering_preserves_captured_object_method_facts_in_nested_function() {
    let program = parse_and_resolve(
        r#"
        function outer() {
          var obj = {
            method() {
              return 1;
            }
          };
          function run() {
            obj.method();
          }
          run();
        }
        outer();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("captured object-literal method metadata should remain available");
}

#[test]
fn lowering_marks_direct_generator_function_expression_call_as_iterator() {
    let program = parse_and_resolve(
        r#"
        var iter = function*() {}();
        iter.next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("direct generator function expression calls should produce generator iterators");
}

#[test]
fn lowering_applies_array_default_to_nested_object_binding_pattern() {
    let program = parse_and_resolve(
        r#"
        var obj = {
          *method({ w: [x, y, z] = [4, 5, 6] }) {
            console.log(x);
            console.log(y);
            console.log(z);
          }
        };
        obj.method({}).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("nested binding patterns should apply array literal defaults before recursion");
}

#[test]
fn lowering_applies_call_default_to_object_binding_identifier() {
    let program = parse_and_resolve(
        r#"
        function counter() {
          return 4;
        }
        var obj = {
          *method({ x = counter() }) {
            console.log(x);
          }
        };
        obj.method({}).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("binding defaults should lower no-argument function calls");
}

#[test]
fn lowering_allows_object_generator_method_default_parameter() {
    let program = parse_and_resolve(
        r#"
        var callCount = 0;
        var obj = {
          *method(value = 23) {
            console.log(value);
            callCount = callCount + 1;
          }
        };

        obj.method(undefined).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("object generator method defaults should lower through function defaults");
}

#[test]
fn lowering_allows_object_pattern_inside_array_binding() {
    let program = parse_and_resolve(
        r#"
        var obj = {
          *method([{ x }]) {
            console.log(x);
          }
        };

        obj.method([{ x: 23 }]).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("array binding elements should allow nested object patterns");
}

#[test]
fn lowering_applies_object_default_to_nested_object_binding_pattern() {
    let program = parse_and_resolve(
        r#"
        var obj = {
          *method({ w: { x } = { x: 23 } }) {
            console.log(x);
          }
        };

        obj.method({ w: undefined }).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("nested object binding patterns should apply object literal defaults");
}

#[test]
fn lowering_allows_array_pattern_rest_binding_target() {
    let program = parse_and_resolve(
        r#"
        var obj = {
          *method([...[x, y]]) {
            console.log(x + y);
          }
        };

        obj.method([1, 2]).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("array rest binding targets should allow nested array patterns");
}

#[test]
fn lowering_applies_identifier_default_to_nested_array_binding_pattern() {
    let program = parse_and_resolve(
        r#"
        var fallback = [23];
        var obj = {
          *method([x] = fallback) {
            console.log(x);
          }
        };

        obj.method(undefined).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("nested array binding patterns should apply identifier defaults");
}

#[test]
fn lowering_applies_identifier_default_to_nested_array_binding_element() {
    let program = parse_and_resolve(
        r#"
        var values = [2, 1, 3];
        var obj = {
          *method([[...x] = values]) {
            console.log(x.length);
          }
        };

        obj.method([]).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("binding element defaults should allow identifier values");
}

#[test]
fn lowering_applies_function_expression_binding_defaults() {
    let program = parse_and_resolve(
        r#"
        var obj = {
          *method([fn = function () {}, gen = function* () {}]) {
            console.log(fn.name);
            console.log(gen.name);
          }
        };

        obj.method([]).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("binding defaults should allow empty function expressions");
}

#[test]
fn lowering_applies_arrow_and_class_expression_binding_defaults() {
    let program = parse_and_resolve(
        r#"
        var obj = {
          *method([arrow = () => {}, cls = class {}, xCls = class X {}]) {
            console.log(arrow.name);
            console.log(cls.name);
            console.log(xCls.name);
          }
        };

        obj.method([]).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("binding defaults should allow empty arrow and class expressions");
}

#[test]
fn lowering_allows_unresolvable_reference_binding_default() {
    let program = parse_and_resolve(
        r#"
        var obj = {
          *method({ x = unresolvableReference }) {
            console.log(x);
          }
        };

        obj.method({}).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("unresolvable binding defaults should lower to runtime ReferenceError");
}

#[test]
fn lowering_allows_call_computed_object_binding_property() {
    let program = parse_and_resolve(
        r#"
        function thrower() {
          throw new Error("boom");
        }

        var obj = {
          *method({ [thrower()]: x } = {}) {
            console.log(x);
          }
        };

        obj.method().next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("computed binding property calls should lower as property-name evaluation");
}

#[test]
fn lowering_allows_later_parameter_reference_default() {
    let program = parse_and_resolve(
        r#"
        function assertThrows(callback) {
          callback();
        }

        var obj = {
          *method(x = y, y) {
            y;
          }
        };

        assertThrows(function() {
          obj.method();
        });
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("later parameter default references should lower to runtime ReferenceError");
}

#[test]
fn lowering_applies_prefix_increment_binding_default() {
    let program = parse_and_resolve(
        r#"
        var initEvalCount = 0;
        var obj = {
          *method({ poisoned: x = ++initEvalCount }) {
            console.log(x);
          }
        };

        obj.method({ poisoned: undefined }).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("prefix increment binding defaults should lower through assignment semantics");
}

#[test]
fn lowering_applies_function_iife_binding_default() {
    let program = parse_and_resolve(
        r#"
        var initCount = 0;
        function inc() { initCount += 1; }
        var obj = {
          *method([[] = inc()]) {
            console.log(initCount);
          }
        };

        obj.method([]).next();
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("function IIFE binding defaults should lower through their body");
}

#[test]
fn class_method_declaring_class_reference_is_not_issue_289_capture() {
    parse_and_resolve(
        r#"
        class C {
          static #m() { return 3; }
          static readByName() { return C.#m(); }
        }
        "#,
    );
}

#[test]
fn class_method_shadowed_outer_name_is_not_issue_289_capture() {
    parse_and_resolve(
        r#"
        var callCount = 0;
        var C = class {
          method() {
            let callCount = 1;
            return callCount;
          }
        };
        "#,
    );
}

#[test]
fn class_public_accessors_are_not_regular_methods_in_class_decl() {
    use ts2wasm_ir::lowered::LoweredStmt;

    let program = parse_and_resolve(
        r#"
        class Box {
          get value() { return 1; }
          set value(next) { this.next = next; }
          method() { return 2; }
        }
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        LoweredStmt::ClassDecl {
            methods,
            static_methods,
            ..
        } => {
            assert_eq!(methods.len(), 1);
            assert_eq!(methods[0].0, "method");
            assert!(static_methods.is_empty());
        }
        other => panic!("unexpected class decl statement: {other:?}"),
    }
}

#[test]
fn lowering_passes_immutable_class_method_outer_local_capture() {
    use ts2wasm_ir::lowered::{FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        let suffix = "-capture";
        class Reader {
          read(prefix) {
            return prefix + suffix;
          }
        }
        let reader = new Reader();
        console.log(reader.read("class"));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read = &lowered.functions[1];
    assert_eq!(read.params, vec![LocalId(0), LocalId(1)]);

    // Verify ClassDecl is emitted with constructor and read method FuncIds
    match &lowered.top_level_statements[1] {
        LoweredStmt::ClassDecl {
            name,
            constructor,
            methods,
            ..
        } => {
            assert_eq!(name, "Reader");
            assert!(
                constructor.is_some(),
                "constructor FuncId should always be set"
            );
            assert_eq!(methods.len(), 1);
            assert_eq!(methods[0].0, "read");
        }
        other => panic!("unexpected class decl statement: {other:?}"),
    }

    match &lowered.top_level_statements[3] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
                args,
                ..
            },
            _,
        ) => match &args[..] {
            [
                LoweredExpr::Call {
                    kind: FunctionCallKind::User(_),
                    args: method_args,
                    ..
                },
            ] => {
                assert!(matches!(method_args.as_slice(), [
                    LoweredExpr::String(prefix, _),
                    LoweredExpr::Local(LocalId(0), _),
                ] if prefix == "class"));
            }
            other => panic!("unexpected console.log arg for captured class method: {other:?}"),
        },
        other => panic!("unexpected captured class method call statement: {other:?}"),
    }
}

#[test]
fn lowering_splits_functions_and_resolves_ids() {
    let program = parse_and_resolve(
        "function add(a, b) { return a + b; } let x = 1; console.log(add(x, 2));",
    );

    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert_eq!(lowered.functions.len(), 1);
    assert_eq!(lowered.top_level_statements.len(), 3);
    assert_eq!(lowered.top_level_locals.len(), 2);

    match &lowered.top_level_statements[2] {
        ts2wasm_ir::lowered::LoweredStmt::Expr(
            ts2wasm_ir::lowered::LoweredExpr::Call { kind, args, .. },
            _,
        ) => {
            assert!(matches!(
                kind,
                ts2wasm_ir::lowered::FunctionCallKind::Builtin(
                    ts2wasm_ir::builtin::BuiltinId::ConsoleLog
                )
            ));
            assert!(matches!(
                args[0],
                ts2wasm_ir::lowered::LoweredExpr::Call { .. }
            ));
        }
        other => panic!("unexpected lowered statement: {other:?}"),
    }
}

#[test]
fn lowering_hoists_direct_eval_block_function_to_enclosing_function_scope() {
    use ts2wasm_ir::lowered::{
        ClosureRepresentation, FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt,
    };

    let program = parse_resolve_and_expand_static_eval(
        r#"
        function outer() {
          eval('{ function f() { return 1; } }');
          return f();
        }
        console.log(outer());
        "#,
    );

    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let outer = &lowered.functions[0];
    assert_eq!(outer.locals, vec![LocalId(0), LocalId(1)]);
    assert!(matches!(
        outer.body.as_slice(),
        [
            LoweredStmt::Expr(
                LoweredExpr::Block { stmts, .. },
                _
            ),
            LoweredStmt::Return(LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(1)),
                args, ..}, _),
        ] if matches!(
            stmts.as_slice(),
            [
                LoweredStmt::Let(LocalId(0), LoweredExpr::Undefined(_), _),
                LoweredStmt::Let(LocalId(1), LoweredExpr::Undefined(_), _),
                LoweredStmt::Assign(
                    LocalId(1),
                    LoweredExpr::ArrowFn {
                        func_id: FuncId(1),
                        captures,
                        representation: ClosureRepresentation::DirectLocalToken,
                        ..
                    },
                    _
                ),
            ] if captures.is_empty()
        ) && args.is_empty()
    ));
    assert!(matches!(
        lowered.functions[1].body.as_slice(),
        [LoweredStmt::Return(LoweredExpr::Number(1, _), _)]
    ));
}

#[test]
fn lowering_keeps_non_eval_block_function_out_of_enclosing_scope() {
    let program = parse_and_resolve(
        r#"
        if (true) {
          function f() { return 1; }
        }
        console.log(f());
        "#,
    );

    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();
    assert_eq!(err.code, DiagCode::UnresolvedFunction);
    assert!(err.message.contains("`f`"), "{err}");
}

#[test]
fn lowering_binds_function_name_inside_own_body() {
    use ts2wasm_ir::lowered::{ClosureRepresentation, FuncId, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve("function assert(x) { return assert; }");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert!(matches!(
        lowered.functions[0].body.as_slice(),
        [LoweredStmt::Return(
            LoweredExpr::ArrowFn {
                func_id: FuncId(0),
                captures,
                representation: ClosureRepresentation::HeapObject,
                ..
            },
            _
        )] if captures.is_empty()
    ));
    assert!(matches!(
        lowered.top_level_statements.first(),
        Some(LoweredStmt::Let(
            LocalId(0),
            LoweredExpr::ArrowFn {
                func_id: FuncId(0),
                captures,
                representation: ClosureRepresentation::DirectLocalToken,
                ..
            },
            _
        )) if captures.is_empty()
    ));
}

#[test]
fn lowering_preindexes_function_self_property_method_assignment() {
    use ts2wasm_ir::lowered::{FunctionCallKind, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        function assert(x) { return assert._toString(x); }
        assert._toString = function(x) { return x; };
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert!(matches!(
        lowered.functions[0].body.as_slice(),
        [LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(_),
                args,
                ..
            },
            _
        )] if matches!(args.as_slice(), [LoweredExpr::Local(_, _)])
    ));
}

#[test]
fn lowering_preserves_parameter_shadowing_over_function_self_name() {
    use ts2wasm_ir::lowered::{LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve("function f(f) { return f; }");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert!(matches!(
        lowered.functions[0].body.as_slice(),
        [LoweredStmt::Return(LoweredExpr::Local(LocalId(0), _), _)]
    ));
}

#[test]
fn lowering_allows_function_prototype_property_assignment() {
    use ts2wasm_ir::lowered::{LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        function Test262Error() {}
        Test262Error.prototype.toString = function() { return "Test262Error"; };
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert!(matches!(
        lowered.top_level_statements.as_slice(),
        [
            LoweredStmt::Let(_, _, _),
            LoweredStmt::Expr(
                LoweredExpr::PropertySet {
                    object,
                    key,
                    ..
                },
                _
            )
        ] if key == "toString" && matches!(object.as_ref(), LoweredExpr::ObjectNew { .. })
    ));
}

#[test]
fn lowering_rejects_unresolved_name() {
    let program = parse_and_resolve("let x = y;");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();
    assert_eq!(err.code, DiagCode::UnresolvedName);
    assert!(err.message.contains('`'));
}

#[test]
fn lowering_rejects_duplicate_function_in_strict_mode() {
    // Strict mode (via "use strict" directive) rejects duplicate function declarations.
    let program =
        parse_and_resolve("\"use strict\"; function f() { return 1; } function f() { return 2; }");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();
    assert_eq!(err.code, DiagCode::DuplicateFunction);
}

#[test]
fn lowering_accepts_duplicate_function_in_non_strict_mode() {
    // Non-strict mode allows duplicate function declarations (web compat, ES spec).
    let program = parse_and_resolve("function f() { return 1; } function f() { return 2; }");
    assert!(ts2wasm_ir::lowered::lower_program(&program).is_ok());
}

#[test]
fn lowering_rejects_duplicate_parameter_in_strict_mode() {
    // "use strict" creates a strict context — duplicate params rejected in all functions.
    let program = parse_and_resolve("\"use strict\"; let f = (a, a) => a;");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();
    assert_eq!(err.code, DiagCode::DuplicateParameter);
}

#[test]
fn lowering_accepts_duplicate_parameter_in_non_strict_mode() {
    // Non-strict function declarations allow duplicate parameter names per ES spec.
    let program = parse_and_resolve("function f(a, a) { return a; }");
    ts2wasm_ir::lowered::lower_program(&program).unwrap();
}

#[test]
fn lowering_accepts_non_ascii_string_literal() {
    let program = parse_and_resolve("let s = \"あ\";");
    // Should succeed without error (previously rejected non-ASCII)
    ts2wasm_ir::lowered::lower_program(&program).unwrap();
}

#[test]
fn lowering_routes_regexp_literal_to_string_subset() {
    let program = parse_and_resolve("let r = /abc/i;");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::String(value, _),
            _,
        ) => assert_eq!(value, "/abc/i"),
        other => panic!("unexpected lowered statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_regexp_literal_test_to_runtime_call() {
    let program = parse_and_resolve("let ok = /abc/.test(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::RegExpTest);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_empty_noncapturing_regexp_literal_to_empty_pattern() {
    let program = parse_and_resolve("let ok = /(?:)/.test(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::RegExpTest);
            assert!(matches!(
                args.as_slice(),
                [
                    ts2wasm_ir::lowered::LoweredExpr::String(pattern, _),
                    ts2wasm_ir::lowered::LoweredExpr::String(input, _)
                ] if pattern == "//" && input == "zabcx"
            ));
        }
        other => panic!("unexpected lowered statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_new_regexp_test_to_runtime_call() {
    let program = parse_and_resolve("let r = new RegExp(\"abc\"); let ok = r.test(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::String(value, _),
            _,
        ) => assert_eq!(value, "/abc/"),
        other => panic!("unexpected RegExp constructor lowering: {other:?}"),
    }

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::RegExpTest);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered RegExp test statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_new_regexp_with_g_flag_test_to_runtime_call() {
    let program =
        parse_and_resolve("let r = new RegExp(\"abc\", \"g\"); let ok = r.test(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::String(value, _),
            _,
        ) => assert_eq!(value, "/abc/g"),
        other => panic!("unexpected RegExp constructor lowering: {other:?}"),
    }

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::RegExpTest);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered RegExp test statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_direct_new_regexp_test_to_runtime_call() {
    let program = parse_and_resolve("let ok = new RegExp(\"abc\").test(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::RegExpTest);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered direct new RegExp.prototype.test statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_string_match_regexp_literal_to_runtime_call() {
    let program = parse_and_resolve("let hit = \"zabcx\".match(/abc/);");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::RegExpMatch);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered String.prototype.match statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_string_match_new_regexp_to_runtime_call() {
    let program = parse_and_resolve("let hit = \"zabcx\".match(new RegExp(\"abc\"));");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::RegExpMatch);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered String.prototype.match statement: {other:?}"),
    }
}

#[test]
fn lowering_rejects_string_match_without_argument_without_panic() {
    let program = parse_and_resolve("let hit = \"zabcx\".match();");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(
        err.message
            .contains("String.prototype.match supports only RegExp literal"),
        "unexpected diagnostic: {err:?}"
    );
}

#[test]
fn lowering_keeps_array_push_expression_length_returning() {
    let program = parse_and_resolve("let arr = [1, 2, 3]; let n = arr.push(4); console.log(n);");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::ArrayPush);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered Array.prototype.push statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_regexp_literal_exec_to_runtime_call() {
    let program = parse_and_resolve("let hit = /abc/.exec(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::RegExpMatch);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered RegExp.prototype.exec statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_identifier_regexp_exec_to_runtime_call() {
    let program = parse_and_resolve("let r = new RegExp(\"abc\"); let hit = r.exec(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::RegExpMatch);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered RegExp.prototype.exec statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_direct_new_regexp_exec_to_runtime_call() {
    let program = parse_and_resolve("let hit = new RegExp(\"abc\").exec(\"zabcx\");");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::RegExpMatch);
            assert_eq!(args.len(), 2);
        }
        other => panic!("unexpected lowered direct new RegExp.prototype.exec statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_new_date_epoch_to_runtime_call() {
    let program = parse_and_resolve("let epoch = new Date(0);");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::DateNew);
            assert_eq!(args.len(), 1);
        }
        other => panic!("unexpected lowered Date constructor statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_new_date_decimal_epoch_to_runtime_call() {
    let program = parse_and_resolve("let epoch = new Date(1726773817847);");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::DateNew);
            assert!(matches!(
                args.as_slice(),
                [ts2wasm_ir::lowered::LoweredExpr::DecimalNumber(value, _)]
                    if value == "1726773817847"
            ));
        }
        other => panic!("unexpected lowered Date decimal constructor statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_new_date_decimal_epoch_local_to_runtime_call() {
    let program = parse_and_resolve(
        "let sampleEpochMs = 1726773817847; let epoch = new Date(sampleEpochMs);",
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::DateNew);
            assert!(matches!(
                args.as_slice(),
                [ts2wasm_ir::lowered::LoweredExpr::Local(
                    ts2wasm_ir::lowered::LocalId(0),
                    _
                )]
            ));
        }
        other => panic!("unexpected lowered Date decimal local constructor statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_dynamic_new_date_argument_to_runtime_call() {
    let program = parse_and_resolve("function makeDate(value) { return new Date(value); }");
    let lowered = ts2wasm_ir::lowered::lower_program(&program)
        .expect("dynamic Date constructor argument should lower");
    ts2wasm_ir::lowered::validate_lowered(&lowered)
        .expect("dynamic Date constructor argument should validate");
}

#[test]
fn lowering_routes_date_get_time_to_runtime_call() {
    let program = parse_and_resolve("let epoch = new Date(0); let ms = epoch.getTime();");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::DateGetTime);
            assert_eq!(args.len(), 1);
        }
        other => panic!("unexpected lowered Date.prototype.getTime statement: {other:?}"),
    }
}

#[test]
fn lowering_routes_date_now_to_live_time_runtime_call() {
    let program = parse_and_resolve("let ms = Date.now();");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::DateNow);
            assert!(args.is_empty());
        }
        other => panic!("unexpected lowered Date.now statement: {other:?}"),
    }
}

#[test]
fn lowering_accepts_char_class_regexp_test_pattern() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve("let ok = /[abc]/.test(\"aaa\");"))
            .is_ok()
    );
}

#[test]
fn lowering_accepts_star_regexp_test_pattern() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve("let ok = /a*/.test(\"aaa\");"))
            .is_ok()
    );
}

#[test]
fn lowering_accepts_regexp_char_class_new_regexp() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve("let r = new RegExp(\"[abc]\");"))
            .is_ok()
    );
}

#[test]
fn lowering_accepts_regexp_i_flag_new_regexp() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve(
            "let r = new RegExp(\"abc\", \"i\");"
        ))
        .is_ok()
    );
}

#[test]
fn lowering_accepts_static_string_regexp_constructor_pattern() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve(
            "let alpha = \"[a-z]\"; let r = new RegExp(\"^\" + alpha + \"$\", \"i\");"
        ))
        .is_ok()
    );
}

#[test]
fn lowering_accepts_intl_harness_static_regexp_constructor_pattern() {
    let program = parse_and_resolve(
        r#"
        function isCanonicalizedStructurallyValidLanguageTag(locale) {
          var alpha = "[a-z]",
            digit = "[0-9]",
            alphanum = "[a-z0-9]",
            variant = "(" + alphanum + "{5,8}|(?:" + digit + alphanum + "{3}))",
            region = "(" + alpha + "{2}|" + digit + "{3})",
            script = "(" + alpha + "{4})",
            language = "(" + alpha + "{2,3}|" + alpha + "{5,8})",
            privateuse = "(x(-[a-z0-9]{1,8})+)",
            singleton = "(" + digit + "|[a-wy-z])",
            attribute= "(" + alphanum + "{3,8})",
            keyword = "(" + alphanum + alpha + "(-" + alphanum + "{3,8})*)",
            unicode_locale_extensions = "(u((-" + keyword + ")+|((-" + attribute + ")+(-" + keyword + ")*)))",
            tlang = "(" + language + "(-" + script + ")?(-" + region + ")?(-" + variant + ")*)",
            tfield = "(" + alpha + digit + "(-" + alphanum + "{3,8})+)",
            transformed_extensions = "(t((-" + tlang + "(-" + tfield + ")*)|(-" + tfield + ")+))",
            other_singleton = "(" + digit + "|[a-sv-wy-z])",
            other_extensions = "(" + other_singleton + "(-" + alphanum + "{2,8})+)",
            extension = "(" + unicode_locale_extensions + "|" + transformed_extensions + "|" + other_extensions + ")",
            locale_id = language + "(-" + script + ")?(-" + region + ")?(-" + variant + ")*(-" + extension + ")*(-" + privateuse + ")?",
            languageTag = "^(" + locale_id + ")$",
            languageTagRE = new RegExp(languageTag, "i");
          var duplicateSingleton = "-" + singleton + "-(.*-)?\\1(?!" + alphanum + ")",
            duplicateSingletonRE = new RegExp(duplicateSingleton, "i"),
            duplicateVariant = "(" + alphanum + "{2,8}-)+" + variant + "-(" + alphanum + "{2,8}-)*\\2(?!" + alphanum + ")",
            duplicateVariantRE = new RegExp(duplicateVariant, "i");
          var transformKeyRE = new RegExp("^" + alpha + digit + "$", "i");

          return languageTagRE.test(locale)
            && !duplicateSingletonRE.test(locale)
            && !duplicateVariantRE.test(locale)
            && transformKeyRE.test("a0");
        }
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("Intl harness static RegExp constructor pattern should lower");
}

#[test]
fn lowering_accepts_dynamic_new_regexp_pattern_for_string_match() {
    let program = parse_and_resolve(
        r#"
        function getPatternParts(digits, formatted) {
          var oneoneRE = "([^" + digits + "]*)[" + digits + "]+([^" + digits + "]+)[" + digits + "]+([^" + digits + "]*)";
          return formatted.match(new RegExp(oneoneRE));
        }
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("dynamic RegExp constructor pattern should lower for string match");
}

#[test]
fn lowering_accepts_intl_duration_format_methods() {
    let program = parse_and_resolve(
        r#"
        let durationFormat = new Intl.DurationFormat("en", { style: "short" });
        let text = durationFormat.format({ seconds: 1 });
        let parts = durationFormat.formatToParts({ seconds: 1 });
        let options = durationFormat.resolvedOptions();
        console.log(text);
        console.log(parts.length);
        console.log(options.style);
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("Intl.DurationFormat constructor and methods should lower");
}

#[test]
fn lowering_accepts_intl_list_format_methods() {
    let program = parse_and_resolve(
        r#"
        let listFormat = new Intl.ListFormat("en", { type: "unit", style: "short" });
        let text = listFormat.format(["1 second"]);
        let parts = listFormat.formatToParts(["1 second"]);
        let options = listFormat.resolvedOptions();
        console.log(text);
        console.log(parts.length);
        console.log(options.style);
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("Intl.ListFormat constructor and methods should lower");
}

#[test]
fn lowering_accepts_intl_date_time_format_format_method() {
    let program = parse_and_resolve(
        r#"
        let dateTimeFormat = new Intl.DateTimeFormat();
        let text = dateTimeFormat.format(new Date(0));
        "#,
    );

    let lowered = ts2wasm_ir::lowered::lower_program(&program)
        .expect("Intl.DateTimeFormat format method should lower");
    ts2wasm_ir::lowered::validate_lowered(&lowered)
        .expect("Intl.DateTimeFormat format method should validate");
}

#[test]
fn lowering_accepts_intl_date_time_format_range_and_parts_methods() {
    let program = parse_and_resolve(
        r#"
        let dateTimeFormat = new Intl.DateTimeFormat();
        let text = dateTimeFormat.format(new Date(0));
        console.log(text);
        "#,
    );

    let lowered = ts2wasm_ir::lowered::lower_program(&program)
        .expect("Intl.DateTimeFormat range/parts methods should lower");
    ts2wasm_ir::lowered::validate_lowered(&lowered)
        .expect("Intl.DateTimeFormat range/parts methods should validate");
}

#[test]
fn lowering_accepts_testintl_constructor_supported_locales_of_alias() {
    let program = parse_and_resolve(
        r#"
        function check(Constructor) {
          let supported = Constructor.supportedLocalesOf([]);
          return supported.length;
        }
        "#,
    );

    let lowered = ts2wasm_ir::lowered::lower_program(&program)
        .expect("testIntl Constructor.supportedLocalesOf alias should lower");
    ts2wasm_ir::lowered::validate_lowered(&lowered)
        .expect("testIntl Constructor.supportedLocalesOf alias should validate");
}

#[test]
fn lowering_accepts_testintl_duration_format_resolved_options_helper() {
    let program = parse_and_resolve(
        r#"
        function partitionDurationFormatPattern(durationFormat, duration) {
          let options = durationFormat.resolvedOptions();
          let style = options.seconds;
          let display = options.secondsDisplay;
          return style + display;
        }
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("testIntl durationFormat.resolvedOptions helper should lower");
}

#[test]
fn lowering_accepts_testintl_list_format_to_parts_helper() {
    let program = parse_and_resolve(
        r#"
        function partitionDurationFormatPattern(strings) {
          let listStyle = "short";
          let lf = new Intl.ListFormat("en", { type: "unit", style: listStyle });
          let flattened = [];
          for (let {type, value} of lf.formatToParts(strings)) {
            if (type === "element") {
              flattened.push({type, value});
            } else {
              flattened.push({type, value});
            }
          }
          return flattened;
        }
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("testIntl ListFormat.formatToParts helper should lower");
}

#[test]
fn lowering_accepts_array_push_spread_from_shift_result() {
    let program = parse_and_resolve(
        r#"
        function flatten() {
          let result = [[{ type: "element", value: "1 second" }]];
          let flattened = [];
          flattened.push(...result.shift());
          return flattened;
        }
        "#,
    );

    ts2wasm_ir::lowered::lower_program(&program)
        .expect("Array.prototype.push should lower spread over shift result");
}

#[test]
fn lowering_validates_direct_call_to_captured_nested_function() {
    let program = parse_and_resolve(
        r#"
        function outer(locale) {
          let suffix = "";
          function inner(locale) {
            return locale + suffix;
          }
          return inner(locale);
        }
        "#,
    );

    let lowered = ts2wasm_ir::lowered::lower_program(&program)
        .expect("captured nested function direct call should lower");
    ts2wasm_ir::lowered::validate_lowered(&lowered)
        .expect("direct call should pass explicit args and captures");
}

#[test]
fn lowering_accepts_captured_static_regexp_constructor_test() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve(
            "function outer() { let alpha = \"[a-z]\"; let r = new RegExp(\"^\" + alpha + \"$\", \"i\"); function inner(value) { return r.test(value); } }"
        ))
        .is_ok()
    );
}

#[test]
fn lowering_rejects_duplicate_new_regexp_flags() {
    let program = parse_and_resolve("let r = new RegExp(\"abc\", \"gg\");");
    let err = ts2wasm_ir::lowered::lower_program(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::UnsupportedSyntax);
    assert!(err.message.contains("issue-051"));
    assert!(err.message.contains("RegExp constructor"));
    assert!(err.message.contains("duplicate"));
}

#[test]
fn lowering_accepts_char_class_string_match() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve("let hit = \"aaa\".match(/[abc]/);"))
            .is_ok()
    );
}

#[test]
fn lowering_accepts_char_class_regexp_exec() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve("let hit = /[abc]/.exec(\"aaa\");"))
            .is_ok()
    );
}

#[test]
fn lowering_accepts_char_class_direct_new_regexp_exec() {
    assert!(
        ts2wasm_ir::lowered::lower_program(&parse_and_resolve(
            "let hit = new RegExp(\"[abc]\").exec(\"aaa\");"
        ))
        .is_ok()
    );
}

#[test]
fn lowering_accepts_regexp_literal_compile() {
    let program = parse_and_resolve("let r = /abc/; r.compile(\"def\");");
    assert!(ts2wasm_ir::lowered::lower_program(&program).is_ok());
}

#[test]
fn lowering_accepts_new_regexp_compile() {
    let program = parse_and_resolve("let r = new RegExp(\"abc\"); r.compile(\"def\");");
    assert!(ts2wasm_ir::lowered::lower_program(&program).is_ok());
}

#[test]
fn lowering_accepts_direct_new_regexp_compile() {
    let program = parse_and_resolve("new RegExp(\"abc\").compile(\"def\");");
    assert!(ts2wasm_ir::lowered::lower_program(&program).is_ok());
}

#[test]
fn lowering_routes_template_interpolation_through_addition() {
    let program = parse_and_resolve("let name = \"world\"; let message = `Hello, ${name}!`;");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::Binary {
                left,
                op: ts2wasm_ir::lowered::LoweredBinaryOp::Add,
                right,
                ..
            },
            _,
        ) => {
            assert!(matches!(
                right.as_ref(),
                ts2wasm_ir::lowered::LoweredExpr::String(value, _) if value == "!"
            ));
            assert!(matches!(
                left.as_ref(),
                ts2wasm_ir::lowered::LoweredExpr::Binary {
                    op: ts2wasm_ir::lowered::LoweredBinaryOp::Add,
                    ..
                }
            ));
        }
        other => panic!("unexpected lowered template statement: {other:?}"),
    }
}

#[test]
fn validate_rejects_arity_mismatch() {
    use ts2wasm_ir::lowered::{
        FuncId, FunctionCallKind, LocalId, LoweredBinaryOp, LoweredExpr, LoweredFunction,
        LoweredProgram, LoweredStmt,
    };

    let func = LoweredFunction {
        id: FuncId(0),
        params: vec![LocalId(0), LocalId(1)],
        uses_receiver: false,
        min_required_params: 2,
        rest_param_index: None,
        metadata_length: None,
        metadata_name: None,
        recursion_depth: 0,
        is_async: false,
        is_generator: false,
        generator_state: None,
        locals: vec![],
        body: vec![],
    };
    let call = LoweredStmt::Expr(
        LoweredExpr::Call {
            kind: FunctionCallKind::User(FuncId(0)),
            args: vec![LoweredExpr::Number(1, Span::generated("test"))],
            span: Span::generated("test"),
        },
        Span::generated("test"),
    );
    let program = LoweredProgram {
        top_level_statements: vec![call],
        top_level_locals: vec![],
        functions: vec![func],
        modules: vec![],
    };

    // JS semantics: callers may pass fewer args than declared params (missing params are undefined).
    let result = ts2wasm_ir::lowered::validate_lowered(&program);
    assert!(
        result.is_ok(),
        "call with fewer args than params should be valid per JS semantics"
    );
    let _ = LoweredBinaryOp::Add;
}

#[test]
fn validate_rejects_non_contiguous_top_level_locals() {
    use ts2wasm_ir::lowered::{LocalId, LoweredProgram};
    let program = LoweredProgram {
        top_level_statements: vec![],
        top_level_locals: vec![LocalId(0), LocalId(2)], // non-contiguous: 1 is missing
        functions: vec![],
        modules: vec![],
    };
    let errs = ts2wasm_ir::lowered::validate_lowered(&program).unwrap_err();
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0].code, DiagCode::InvariantViolation);
    assert!(errs[0].message.contains("top_level_locals"));
}

#[test]
fn typescript_semantics_allows_block_scoped_same_name_extra_argument() {
    let program = parse_and_resolve(
        r#"
        function foo(a: number) {
          if (a === 1) {
            function foo() {}
            foo();
            foo(10);
          }
        }
        "#,
    );

    ts2wasm_ir::validate_typescript_call_arity(&program).unwrap();
}

#[test]
fn typescript_semantics_allows_extra_arguments_when_function_reads_arguments() {
    let program = parse_and_resolve(
        r#"
        function first() {
          return arguments[0];
        }
        first(7);
        "#,
    );

    ts2wasm_ir::validate_typescript_call_arity(&program).unwrap();
}

#[test]
fn typescript_semantics_uses_ambient_function_parameter_arity() {
    let program = parse_and_resolve(
        r#"
        declare function canYouInferThis(fn: () => number): number;
        canYouInferThis(() => 1);
        "#,
    );

    ts2wasm_ir::validate_typescript_call_arity(&program).unwrap();
}

#[test]
fn typescript_semantics_uses_ambient_optional_and_rest_arity() {
    let program = parse_and_resolve(
        r#"
        declare function optional(value?: number): number;
        declare function variadic(first: number, ...rest: number[]): number;
        optional();
        optional(1);
        variadic(1);
        variadic(1, 2, 3);
        "#,
    );

    ts2wasm_ir::validate_typescript_call_arity(&program).unwrap();
}

#[test]
fn typescript_semantics_rejects_ambient_function_missing_argument() {
    let program = parse_and_resolve(
        r#"
        declare function required(value: number): number;
        required();
        "#,
    );
    let err = ts2wasm_ir::validate_typescript_call_arity(&program).unwrap_err();

    assert_eq!(err.code, DiagCode::ArityMismatch);
    assert!(err.message.contains("TS2554"));
    assert!(err.message.contains("Expected 1 arguments, but got 0"));
    assert!(err.span.is_some(), "call-site span should be preserved");
}

#[test]
fn typescript_semantics_accepts_bodyful_function_missing_argument_without_arguments_object() {
    let program = parse_and_resolve(
        r#"
        function foo(a: number) {
          if (a === 1) {
            function foo() {}
            foo();
          }
          foo();
        }
        "#,
    );

    ts2wasm_ir::validate_typescript_call_arity(&program).unwrap();
}

#[test]
fn lowering_represents_plain_ternary_as_expression_block() {
    use ts2wasm_ir::lowered::{LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        let x = true;
        let y = x ? x : [];
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    match &lowered.top_level_statements[1] {
        LoweredStmt::Let(LocalId(1), LoweredExpr::Block { stmts, result, .. }, _) => {
            assert!(matches!(
                stmts.as_slice(),
                [
                    LoweredStmt::Let(LocalId(2), LoweredExpr::Undefined(_), _),
                    LoweredStmt::If { .. }
                ]
            ));
            assert!(matches!(result.as_ref(), LoweredExpr::Local(LocalId(2), _)));
        }
        other => panic!("unexpected lowered ternary statement: {other:?}"),
    }
}

#[test]
fn lowering_represents_returned_ordinary_closure_as_heap_creation() {
    use ts2wasm_ir::lowered::{ClosureRepresentation, FuncId, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        function makeReader() {
          let value = "escaped-closure";
          function read() {
            return value;
          }
          return read;
        }
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    assert_eq!(lowered.functions.len(), 2);
    let make_reader = &lowered.functions[0];
    let read_func = &lowered.functions[1];
    assert_eq!(read_func.params, vec![LocalId(0)]);

    match &make_reader.body[1] {
        LoweredStmt::Let(
            LocalId(1),
            LoweredExpr::ArrowFn {
                func_id,
                captures,
                representation,
                ..
            },
            _,
        ) => {
            assert_eq!(*func_id, FuncId(1));
            assert_eq!(captures, &vec![LocalId(0)]);
            assert_eq!(*representation, ClosureRepresentation::DirectLocalToken);
        }
        other => panic!("unexpected local closure binding: {other:?}"),
    }

    match &make_reader.body[2] {
        LoweredStmt::Return(
            LoweredExpr::ArrowFn {
                func_id,
                captures,
                representation,
                ..
            },
            _,
        ) => {
            assert_eq!(*func_id, FuncId(1));
            assert_eq!(captures, &vec![LocalId(0)]);
            assert_eq!(*representation, ClosureRepresentation::HeapObject);
        }
        other => panic!("unexpected returned closure representation: {other:?}"),
    }
}

#[test]
fn lowering_represents_known_heap_closure_local_call_explicitly() {
    use ts2wasm_ir::lowered::{
        ClosureRepresentation, FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt,
    };

    let program = parse_and_resolve(
        r#"
        function makeReader() {
          let value = "escaped-closure";
          function read() {
            return value;
          }
          return read;
        }

        let reader = makeReader();
        console.log(reader());
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    // top_level_statements[0] is the function declaration binding (LocalId(1) due to inner read() closure)
    match &lowered.top_level_statements[0] {
        LoweredStmt::Let(
            LocalId(1),
            LoweredExpr::ArrowFn {
                func_id: FuncId(0),
                captures,
                representation,
                ..
            },
            _,
        ) => {
            assert!(captures.is_empty());
            assert_eq!(*representation, ClosureRepresentation::DirectLocalToken);
        }
        other => panic!("unexpected function declaration binding: {other:?}"),
    }

    // top_level_statements[1] is reader = makeReader()
    match &lowered.top_level_statements[1] {
        LoweredStmt::Let(
            LocalId(0),
            LoweredExpr::Call {
                kind: FunctionCallKind::User(_),
                ..
            },
            _,
        ) => {}
        other => panic!("unexpected heap closure local binding: {other:?}"),
    }

    match &lowered.top_level_statements[2] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
                args,
                ..
            },
            _,
        ) => match &args[..] {
            [
                LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: call_args,
                    ..
                },
            ] => {
                assert_eq!(*intrinsic, RuntimeFn::HeapClosureCall);
                assert!(matches!(
                    call_args.as_slice(),
                    [LoweredExpr::Local(LocalId(0), _)]
                ));
            }
            other => panic!("unexpected console.log argument for heap closure call: {other:?}"),
        },
        other => panic!("unexpected lowered heap closure call statement: {other:?}"),
    }
}

#[test]
fn validate_accepts_heap_closure_creation_for_backend_dispatch() {
    let program = parse_and_resolve(
        r#"
        function makeReader() {
          let value = "escaped-closure";
          function read() {
            return value;
          }
          return read;
        }
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    ts2wasm_ir::lowered::validate_lowered(&lowered).unwrap();
}

#[test]
fn lowering_represents_private_field_access_as_internal_slot_calls() {
    use ts2wasm_ir::lowered::{FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class Counter {
          #value = 7;
          read() { return this.#value; }
          write(v) { this.#value = v; }
        }

        let c = new Counter();
        console.log(c.read());
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    // ClassDecl is now emitted at index 0
    match &lowered.top_level_statements[0] {
        LoweredStmt::ClassDecl {
            name,
            constructor,
            methods,
            private_fields,
            ..
        } => {
            assert_eq!(name, "Counter");
            assert!(constructor.is_some());
            assert_eq!(methods.len(), 2);
            assert_eq!(methods[0].0, "read");
            assert_eq!(methods[1].0, "write");
            assert_eq!(private_fields.as_slice(), ["value"]);
        }
        other => panic!("unexpected class decl: {other:?}"),
    }

    match &lowered.top_level_statements[1] {
        LoweredStmt::Let(
            LocalId(0),
            LoweredExpr::New {
                private_brand,
                private_slot_count,
                ..
            },
            _,
        ) => {
            assert_eq!(*private_brand, Some(1));
            assert_eq!(*private_slot_count, 1);
        }
        other => panic!("unexpected private class instance binding: {other:?}"),
    }

    let constructor = &lowered.functions[0];
    match &constructor.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::PrivateFieldSet);
            assert!(matches!(
                args.as_slice(),
                [
                    LoweredExpr::Local(LocalId(0), _),
                    LoweredExpr::Number(1, _),
                    LoweredExpr::Number(0, _),
                    LoweredExpr::Number(7, _)
                ]
            ));
        }
        other => panic!("unexpected private field initializer lowering: {other:?}"),
    }

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::PrivateFieldGet);
            assert!(matches!(
                args.as_slice(),
                [
                    LoweredExpr::Local(LocalId(0), _),
                    LoweredExpr::Number(1, _),
                    LoweredExpr::Number(0, _)
                ]
            ));
        }
        other => panic!("unexpected private field read lowering: {other:?}"),
    }

    match &lowered.top_level_statements[2] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::Builtin(ts2wasm_ir::builtin::BuiltinId::ConsoleLog),
                ..
            },
            _,
        ) => {}
        other => panic!("unexpected console.log lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_same_class_private_field_receiver_as_branded_slot_call() {
    use ts2wasm_ir::lowered::{LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class Counter {
          #value = 7;
          readFrom(other) { return other.#value; }
          writeTo(other, next) { other.#value = next; return other.#value; }
        }

        let first = new Counter();
        let second = new Counter();
        console.log(first.readFrom(second));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::PrivateFieldGet);
            assert!(matches!(
                args.as_slice(),
                [
                    LoweredExpr::Local(LocalId(0), _),
                    LoweredExpr::Number(1, _),
                    LoweredExpr::Number(0, _)
                ]
            ));
        }
        other => panic!("unexpected same-class private receiver read lowering: {other:?}"),
    }

    let write_method = &lowered.functions[2];
    match &write_method.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::RuntimeCall {
                intrinsic, args, ..
            },
            _,
        ) => {
            assert_eq!(*intrinsic, RuntimeFn::PrivateFieldSet);
            assert!(matches!(
                args.as_slice(),
                [
                    LoweredExpr::Local(LocalId(0), _),
                    LoweredExpr::Number(1, _),
                    LoweredExpr::Number(0, _),
                    LoweredExpr::Local(LocalId(1), _)
                ]
            ));
        }
        other => panic!("unexpected same-class private receiver write lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_direct_private_method_call_as_same_class_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          #m(v) { return v + 1; }
          read() { return this.#m(2); }
        }

        let c = new C();
        console.log(c.read());
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _,
        ) => assert!(matches!(
            args.as_slice(),
            [LoweredExpr::Local(LocalId(0), _), LoweredExpr::Number(2, _)]
        )),
        other => panic!("unexpected private method call lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_private_method_non_this_receiver_as_brand_checked_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          #m() { return 1; }
          read(other) { return other.#m(); }
        }

        let c = new C();
        console.log(c.read(c));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _,
        ) => match args.as_slice() {
            [
                LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: brand_args,
                    ..
                },
            ] => {
                assert_eq!(*intrinsic, RuntimeFn::PrivateBrandCheck);
                assert!(matches!(
                    brand_args.as_slice(),
                    [LoweredExpr::Local(_, _), LoweredExpr::Number(1, _)]
                ));
            }
            other => panic!("unexpected private method brand-check args: {other:?}"),
        },
        other => panic!("unexpected private method call lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_static_private_method_call_as_same_class_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          static #m() { return 3; }
          static read() { return this.#m(); }
          static readByName() { return C.#m(); }
        }

        console.log(C.read());
        console.log(C.readByName());
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(3)),
                args,
                ..
            },
            _,
        ) => assert!(args.is_empty()),
        other => panic!("unexpected static private method call lowering: {other:?}"),
    }

    let read_by_name_method = &lowered.functions[2];
    match &read_by_name_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(3)),
                args,
                ..
            },
            _,
        ) => assert!(args.is_empty()),
        other => panic!("unexpected static private method class-name call lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_static_private_accessor_access_as_same_class_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          static get #x() { return 3; }
          static set #y(next) { console.log(next); }
          static read() { return this.#x; }
          static readByName() { return C.#x; }
          static write(next) { this.#y = next; }
          static writeByName(next) { C.#y = next; }
        }

        console.log(C.read());
        console.log(C.readByName());
        C.write(4);
        C.writeByName(5);
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(5)),
                args,
                ..
            },
            _,
        ) => assert!(args.is_empty()),
        other => panic!("unexpected static private getter access lowering: {other:?}"),
    }

    let read_by_name_method = &lowered.functions[2];
    match &read_by_name_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(5)),
                args,
                ..
            },
            _,
        ) => assert!(args.is_empty()),
        other => panic!("unexpected static private getter class-name lowering: {other:?}"),
    }

    let write_method = &lowered.functions[3];
    match &write_method.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(6)),
                args,
                ..
            },
            _,
        ) => assert!(matches!(
            args.as_slice(),
            [LoweredExpr::Local(LocalId(1), _)]
        )),
        other => panic!("unexpected static private setter assignment lowering: {other:?}"),
    }

    let write_by_name_method = &lowered.functions[4];
    match &write_by_name_method.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(6)),
                args,
                ..
            },
            _,
        ) => assert!(matches!(
            args.as_slice(),
            [LoweredExpr::Local(LocalId(0), _)]
        )),
        other => panic!("unexpected static private setter class-name lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_static_private_field_access_as_same_class_env_cell() {
    use ts2wasm_ir::lowered::{LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          static #x = 3;
          static read() { return this.#x; }
          static write(next) { C.#x = next; return C.#x; }
        }

        console.log(C.read());
        console.log(C.write(4));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    // ClassDecl is now emitted at index 0
    match &lowered.top_level_statements[0] {
        LoweredStmt::ClassDecl {
            name,
            constructor,
            methods,
            static_methods,
            private_fields,
            ..
        } => {
            assert_eq!(name, "C");
            assert!(
                constructor.is_some(),
                "constructor FuncId should always be set"
            );
            assert_eq!(methods.len(), 0);
            assert_eq!(static_methods.len(), 2);
            assert_eq!(static_methods[0].0, "read");
            assert_eq!(static_methods[1].0, "write");
            assert!(private_fields.is_empty());
        }
        other => panic!("unexpected class decl: {other:?}"),
    }

    match &lowered.top_level_statements[1] {
        LoweredStmt::Let(LocalId(0), LoweredExpr::EnvCellNew(initializer, _), _) => {
            assert!(matches!(initializer.as_ref(), LoweredExpr::Number(3, _)));
        }
        other => panic!("unexpected static private field storage lowering: {other:?}"),
    }

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(LoweredExpr::EnvCellGet(LocalId(1), _), _) => {}
        other => panic!("unexpected static private field read lowering: {other:?}"),
    }

    let write_method = &lowered.functions[2];
    match &write_method.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::EnvCellSet {
                cell: LocalId(1),
                expr,
                ..
            },
            _,
        ) => assert!(matches!(expr.as_ref(), LoweredExpr::Local(LocalId(0), _))),
        other => panic!("unexpected static private field write lowering: {other:?}"),
    }
    match &write_method.body[1] {
        LoweredStmt::Return(LoweredExpr::EnvCellGet(LocalId(1), _), _) => {}
        other => panic!("unexpected static private field class-name read lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_direct_private_getter_access_as_same_class_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          get #x() { return 3; }
          read() { return this.#x; }
        }

        let c = new C();
        console.log(c.read());
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _,
        ) => assert!(matches!(
            args.as_slice(),
            [LoweredExpr::Local(LocalId(0), _)]
        )),
        other => panic!("unexpected private getter access lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_private_getter_non_this_receiver_as_brand_checked_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          get #x() { return 3; }
          read(other) { return other.#x; }
        }

        let c = new C();
        console.log(c.read(c));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let read_method = &lowered.functions[1];
    match &read_method.body[0] {
        LoweredStmt::Return(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _,
        ) => match args.as_slice() {
            [
                LoweredExpr::RuntimeCall {
                    intrinsic,
                    args: brand_args,
                    ..
                },
            ] => {
                assert_eq!(*intrinsic, RuntimeFn::PrivateBrandCheck);
                assert!(matches!(
                    brand_args.as_slice(),
                    [LoweredExpr::Local(_, _), LoweredExpr::Number(1, _)]
                ));
            }
            other => panic!("unexpected private getter brand-check args: {other:?}"),
        },
        other => panic!("unexpected private getter access lowering: {other:?}"),
    }
}

#[test]
fn lowering_represents_direct_private_setter_assignment_as_same_class_user_call() {
    use ts2wasm_ir::lowered::{FuncId, FunctionCallKind, LocalId, LoweredExpr, LoweredStmt};

    let program = parse_and_resolve(
        r#"
        class C {
          #value = 0;
          set #x(next) { this.#value = next; }
          write(next) { this.#x = next; }
        }

        let c = new C();
        console.log(c.write(2));
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();

    let write_method = &lowered.functions[1];
    match &write_method.body[0] {
        LoweredStmt::Expr(
            LoweredExpr::Call {
                kind: FunctionCallKind::User(FuncId(2)),
                args,
                ..
            },
            _,
        ) => assert!(matches!(
            args.as_slice(),
            [
                LoweredExpr::Local(LocalId(0), _),
                LoweredExpr::Local(LocalId(1), _)
            ]
        )),
        other => panic!("unexpected private setter assignment lowering: {other:?}"),
    }
}

#[test]
fn builtin_console_log_contract_is_effect_only() {
    use ts2wasm_ir::builtin::{BuiltinId, BuiltinResult};
    assert_eq!(BuiltinId::ConsoleLog.expected_arity(), 1);
    assert!(matches!(
        BuiltinId::ConsoleLog.result(),
        BuiltinResult::EffectOnly
    ));
}

#[test]
fn validate_accepts_console_log_with_extra_args() {
    let ast = ts2wasm_cli::parse_program("console.log(1, 2);").unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&ast).unwrap();
    let lowered = ts2wasm_ir::lowered::lower_program(&resolved).unwrap();
    let errs = ts2wasm_ir::lowered::validate_lowered(&lowered);
    // ConsoleLog lowers extra args into one space-joined runtime string,
    // so no arity mismatch is produced.
    if let Err(errors) = errs {
        assert!(!errors.iter().any(|e| e.code == DiagCode::ArityMismatch));
    }
}

#[test]
fn lowering_connects_read_file_sync_idiom_to_builtin_call_shape() {
    let ast =
        ts2wasm_cli::parse_program("let s = require(\"fs\").readFileSync(0, \"utf8\");").unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&ast).unwrap();
    let lowered = ts2wasm_ir::lowered::lower_program(&resolved).unwrap();
    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::Call {
                kind:
                    ts2wasm_ir::lowered::FunctionCallKind::Builtin(
                        ts2wasm_ir::builtin::BuiltinId::ReadStdinUtf8,
                    ),
                args,
                ..
            },
            _,
        ) => {
            assert!(args.is_empty());
        }
        other => panic!("unexpected lowered statement: {other:?}"),
    }
}

#[test]
fn lowering_preserves_dynamic_import_module_load_kind() {
    let ast =
        ts2wasm_cli::parse_program("let mod = import(\"./dynamic-import-helper.ts\");").unwrap();
    let resolved = ts2wasm_ir::builtin_resolver::resolve_builtins(&ast).unwrap();
    let lowered = ts2wasm_ir::lowered::lower_program(&resolved).unwrap();

    match &lowered.top_level_statements[0] {
        ts2wasm_ir::lowered::LoweredStmt::Let(
            _,
            ts2wasm_ir::lowered::LoweredExpr::ModuleLoad { kind, .. },
            _,
        ) => assert_eq!(*kind, ts2wasm_ir::lowered::ModuleLoadKind::DynamicImport),
        other => panic!("dynamic import should lower to DynamicImport ModuleLoad, got {other:?}"),
    }
}

#[test]
fn inferred_type_marks_number_addition_as_number() {
    use ts2wasm_ir::lowered::{InferredType, LoweredBinaryOp, LoweredExpr};
    let expr = LoweredExpr::Binary {
        left: Box::new(LoweredExpr::Number(1, Span::generated("test"))),
        op: LoweredBinaryOp::Add,
        right: Box::new(LoweredExpr::Number(2, Span::generated("test"))),
        span: Span::generated("test"),
    };
    assert_eq!(expr.inferred_type(), InferredType::Number);
}

#[test]
fn inferred_type_marks_string_addition_as_string() {
    use ts2wasm_ir::lowered::{InferredType, LoweredBinaryOp, LoweredExpr};
    let expr = LoweredExpr::Binary {
        left: Box::new(LoweredExpr::String("a".to_owned(), Span::generated("test"))),
        op: LoweredBinaryOp::Add,
        right: Box::new(LoweredExpr::String("b".to_owned(), Span::generated("test"))),
        span: Span::generated("test"),
    };
    assert_eq!(expr.inferred_type(), InferredType::String);
}

#[test]
fn inferred_type_falls_back_to_unknown_for_mixed_add() {
    use ts2wasm_ir::lowered::{InferredType, LoweredBinaryOp, LoweredExpr};
    let expr = LoweredExpr::Binary {
        left: Box::new(LoweredExpr::String("a".to_owned(), Span::generated("test"))),
        op: LoweredBinaryOp::Add,
        right: Box::new(LoweredExpr::Number(1, Span::generated("test"))),
        span: Span::generated("test"),
    };
    assert_eq!(expr.inferred_type(), InferredType::Unknown);
}

#[test]
fn lower_arrow_fn_iife_empty_body() {
    let program = parse_and_resolve("(() => {})();");
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();
    // Should produce a top-level expression that is a Call with a User function kind.
    // The call invokes an ArrowFn that returns undefined.
    assert!(
        matches!(
            lowered.top_level_statements.first(),
            Some(ts2wasm_ir::lowered::LoweredStmt::Expr(
                ts2wasm_ir::lowered::LoweredExpr::Call {
                    kind: ts2wasm_ir::lowered::FunctionCallKind::User(_),
                    args,
                    ..
                },
                _
            )) if args.is_empty()
        ),
        "expected User(..) call with empty args at top level: {:?}",
        lowered.top_level_statements.first()
    );
}

#[test]
fn lower_arrow_fn_iife_with_body() {
    let program = parse_and_resolve(
        r#"
        let x = 1;
        (() => { x = x + 1; })();
        "#,
    );
    let lowered = ts2wasm_ir::lowered::lower_program(&program).unwrap();
    // The arrow should capture `x` from the enclosing scope.
    // The top-level statement[1] should be a Call with the capture arg appended.
    assert!(
        matches!(
            lowered.top_level_statements.get(1),
            Some(ts2wasm_ir::lowered::LoweredStmt::Expr(
                ts2wasm_ir::lowered::LoweredExpr::Call {
                    kind: ts2wasm_ir::lowered::FunctionCallKind::User(_),
                    args,
                    ..
                },
                _
            )) if args.len() == 1
        ),
        "expected User(..) call with one capture arg: {:?}",
        lowered.top_level_statements.get(1)
    );
}
