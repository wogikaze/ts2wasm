// covers: I-20260515-PMTJTQ
// Comma expression in statement position after assignment — parser gap
// expected: [UnsupportedSyntax/parser] expected Semicolon, got Some(Comma)
let a = 1, b = 2;
a, b;
