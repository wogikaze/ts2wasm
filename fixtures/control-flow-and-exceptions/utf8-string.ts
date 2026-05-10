// UTF-8 string operations with multi-byte characters
// Japanese: 日本語 (ni-hon-go) = 3 code points, 9 bytes in UTF-8

// Test .length with multi-byte characters
console.log("日本語".length);        // Should be 3, currently returns 9 (byte count)
console.log("hello".length);         // Should still work: 5
console.log("★".length);             // Star: 1 code point, 3 bytes

// Test charAt with multi-byte characters
console.log("日本語".charAt(0));     // Should be "日"
console.log("日本語".charAt(1));     // Should be "本"
console.log("日本語".charAt(2));     // Should be "語"

// Test at with multi-byte characters
console.log("日本語".at(0));         // Should be "日"
console.log("日本語".at(-1));        // Should be "語"

// Test charCodeAt with multi-byte characters
// 日 = 26085, 本 = 26412, 語 = 35486
let c0 = "日本語".charCodeAt(0);
if (c0 === 26085) { console.log(1); } else { console.log(0); }

let c1 = "日本語".charCodeAt(1);
if (c1 === 26412) { console.log(1); } else { console.log(0); }

let c2 = "日本語".charCodeAt(2);
if (c2 === 35486) { console.log(1); } else { console.log(0); }

// Test string indexing with multi-byte characters
console.log("日本語"[0]);            // Should be "日"
console.log("日本語"[1]);            // Should be "本"
console.log("日本語"[2]);            // Should be "語"

// Test mixed ASCII and multi-byte
console.log("a日本語b".length);      // Should be 5 (a + 3 chars + b)
console.log("a日本語b".charAt(0));   // Should be "a"
console.log("a日本語b".charAt(3));   // Should be "語"
console.log("a日本語b".charAt(4));   // Should be "b"
