// Test fixture for heap OOM check.
// Repeated doubling quickly exceeds the bounded runtime memory limit and must trap.
let s = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
let i = 0;
while (i < 25) {
    s = s + s;
    i = i + 1;
}
console.log(s.length);
