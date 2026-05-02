// Class name used as a pure value (not in new/member access)
// This should fail to compile with issue-5011
class C {
  method() {
    return 42;
  }
}
const y = C;
