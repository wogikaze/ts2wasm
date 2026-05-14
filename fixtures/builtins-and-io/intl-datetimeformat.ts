let dtf = new Intl.DateTimeFormat("en-US", { timeZone: "UTC", localeMatcher: "lookup" });
console.log(dtf.format(0));

let parts = dtf.formatToParts(86400000);
console.log(parts.length);
console.log(parts[0].type);
console.log(parts[0].value);
console.log(parts[4].type);
console.log(parts[4].value);

let options = dtf.resolvedOptions();
console.log(options.locale);
console.log(options.timeZone);
console.log(options.localeMatcher);

let callable = Intl.DateTimeFormat("en-GB", { timeZone: "UTC" });
console.log(callable.format(0));
