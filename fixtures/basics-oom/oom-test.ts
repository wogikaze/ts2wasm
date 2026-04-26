// Test fixture for heap OOM check
// This attempts to allocate more memory than available, which should trap gracefully

// Try to allocate a very large string that exceeds the 2-page (128KB) memory limit
// Initial memory is 2 pages = 128KB
// HEAP_START = 2048, so available heap space is ~126KB
// This allocation should trigger an OOM trap

// Create a large string by concatenating many times
let s = "x";
let i = 0;
while (i < 10000) {
    s = s + "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    i = i + 1;
}
console.log(s);
