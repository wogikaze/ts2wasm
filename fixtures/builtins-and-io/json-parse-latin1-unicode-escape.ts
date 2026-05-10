// JSON.parse unicode escape: Latin-1 range (U+0000-U+00FF)
// The runtime uses single-byte-per-character storage, so \uXXXX for
// Latin-1 characters (e.g., \u00e9 = é) should produce 1 code unit.

// ASCII range: \u0041 = 'A'
let s1 = JSON.parse('"\\u0041"');
console.log(s1.length);
console.log(s1.charCodeAt(0));

// Latin-1 supplement: \u00e9 = 'é' (U+00E9)
let s2 = JSON.parse('"\\u00e9"');
console.log(s2.length);
console.log(s2.charCodeAt(0));

// Mixed Latin-1: \u0041\u00e9 = "Aé"
let s3 = JSON.parse('"\\u0041\\u00e9"');
console.log(s3.length);
console.log(s3.charCodeAt(0));
console.log(s3.charCodeAt(1));

// \u00f1 = 'ñ' (U+00F1)
let s4 = JSON.parse('"\\u00f1"');
console.log(s4.length);
console.log(s4.charCodeAt(0));
