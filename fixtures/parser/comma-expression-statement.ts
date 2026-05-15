// covers: I-20260515-PMTJTQ
// Comma expression in statement position after assignment — parser gap
let a = 1, b = 2, c = 3;
a = 2, b = c, c = 0;
