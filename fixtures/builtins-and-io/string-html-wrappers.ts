// Test: Annex B HTML wrapper string methods

// Methods without attribute args
console.log("hello".bold());
console.log("hello".italics());
console.log("hello".fixed());
console.log("hello".strike());
console.log("hello".small());
console.log("hello".big());
console.log("hello".blink());
console.log("hello".sub());
console.log("hello".sup());

// Methods with attribute args (basic, without special chars)
console.log("hello".anchor("top"));
console.log("hello".link("https://example.com"));
console.log("hello".fontcolor("red"));
console.log("hello".fontsize("12"));

// Edge cases: empty strings
console.log("".bold());
console.log("test".link(""));
console.log("".link("url"));

// Multiple args (extra args ignored)
console.log("x".anchor("a", "extra", "args"));
console.log("x".link("u", "ignored"));
