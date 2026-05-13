let raw = { raw: ["left-", "-right"] };

console.log(String.fromCharCode(65));
console.log(String.fromCodePoint(0x100));
console.log("hello".at(-1));
console.log("AĀB".codePointAt(1));
console.log("locale".toLocaleString());
console.log(String.raw(raw, "middle", "ignored"));
