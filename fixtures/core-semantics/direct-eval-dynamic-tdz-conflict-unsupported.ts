// Expected diagnostic: dynamic direct eval would need an env descriptor that
// models the later `let value` binding's TDZ state. The current host lane only
// exports initialized caller bindings, so it rejects this source instead of
// letting the host eval bypass TDZ.
function f() {
  let source = "value";
  console.log(eval(source));
  let value = 1;
  console.log(value);
}

f();
