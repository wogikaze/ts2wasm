// Parser-level `with` statement support
// The `with` body is extracted as a plain block (`with` semantics are not compiled).
let x = 0;
with (x) {
  let y = 1;
}
