let nf = new Intl.NumberFormat("en-US");
console.log(nf.format(1234));
console.log(nf.format(-987654));

let callable = Intl.NumberFormat("en-US", { notation: "compact", compactDisplay: "short" });
console.log(callable.format(1200));

let currency = new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" });
console.log(currency.format(42));
console.log(currency.resolvedOptions().locale);
console.log(currency.resolvedOptions().numberingSystem);
console.log(currency.resolvedOptions().currency);

let parts = currency.formatToParts(42);
console.log(parts.length);
console.log(parts[0].type);
console.log(parts[0].value);

let signed = new Intl.NumberFormat("en-US", { signDisplay: "always" });
console.log(signed.format(7));
