// Minimal validator.js subset
export function isAlpha(str: string, locale?: string): boolean {
  if (typeof str !== "string") return false;
  let pattern: RegExp;
  if (locale === "en-US" || locale === undefined) {
    pattern = /^[A-Z]+$/i;
  } else if (locale === "de-DE") {
    pattern = /^[A-ZÄÖÜß]+$/i;
  } else if (locale === "ar") {
    pattern = /^[\u0600-\u06FF]+$/;
  } else {
    pattern = /^[A-Z]+$/i;
  }
  return pattern.test(str);
}

export function isAlphanumeric(str: string, locale?: string): boolean {
  if (typeof str !== "string") return false;
  if (locale === "en-US" || locale === undefined) {
    return /^[0-9A-Z]+$/i.test(str);
  }
  return /^[0-9A-Z]+$/i.test(str);
}

export function isNumeric(str: string): boolean {
  if (typeof str !== "string") return false;
  return /^[0-9]+$/.test(str);
}

export function isLowercase(str: string): boolean {
  if (typeof str !== "string") return false;
  return str === str.toLowerCase();
}

export function isUppercase(str: string): boolean {
  if (typeof str !== "string") return false;
  return str === str.toUpperCase();
}

export function isAscii(str: string): boolean {
  if (typeof str !== "string") return false;
  for (let i = 0; i < str.length; i++) {
    if (str.charCodeAt(i) > 127) return false;
  }
  return true;
}

export function isBoolean(str: string): boolean {
  if (typeof str !== "string") return false;
  return str === "true" || str === "false" || str === "1" || str === "0";
}

export function isByteLength(str: string, min: number, max?: number): boolean {
  if (typeof str !== "string") return false;
  const len = str.length;
  if (len < min) return false;
  if (max !== undefined && len > max) return false;
  return true;
}

export function isEmpty(str: string): boolean {
  if (typeof str !== "string") return false;
  return str.length === 0;
}

export function isLength(str: string, min: number, max?: number): boolean {
  if (typeof str !== "string") return false;
  const len = str.length;
  if (len < min) return false;
  if (max !== undefined && len > max) return false;
  return true;
}

export function matches(str: string, pattern: string): boolean {
  if (typeof str !== "string") return false;
  try {
    const re = new RegExp(pattern);
    return re.test(str);
  } catch {
    return false;
  }
}

export function trim(str: string): string {
  if (typeof str !== "string") return "";
  return str.replace(/^\s+/, "").replace(/\s+$/, "");
}

export function escape(str: string): string {
  if (typeof str !== "string") return "";
  return str
    .replace(/&/g, "&amp;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#x27;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

export function ltrim(str: string): string {
  if (typeof str !== "string") return "";
  return str.replace(/^\s+/, "");
}

export function rtrim(str: string): string {
  if (typeof str !== "string") return "";
  return str.replace(/\s+$/, "");
}

export function isEmail(str: string): boolean {
  if (typeof str !== "string") return false;
  // Simplified email regex
  const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return re.test(str);
}

export function isURL(str: string): boolean {
  if (typeof str !== "string") return false;
  const re = /^https?:\/\/[^\s/$.?#].[^\s]*$/i;
  return re.test(str);
}

export function isIP(str: string): boolean {
  if (typeof str !== "string") return false;
  const ipv4 = /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/;
  const m = ipv4.exec(str);
  if (!m) return false;
  for (let i = 1; i <= 4; i++) {
    const n = parseInt(m[i], 10);
    if (n < 0 || n > 255) return false;
  }
  return true;
}

export function isHexColor(str: string): boolean {
  if (typeof str !== "string") return false;
  return /^#?([0-9A-F]{3}|[0-9A-F]{6})$/i.test(str);
}

export function isInt(str: string): boolean {
  if (typeof str !== "string") return false;
  return /^-?[0-9]+$/.test(str);
}

export function isFloat(str: string): boolean {
  if (typeof str !== "string") return false;
  return /^-?[0-9]+(?:\.[0-9]+)?$/.test(str);
}

export function isJSON(str: string): boolean {
  if (typeof str !== "string") return false;
  const s = str.trim();
  if (s === "") return false;
  return (s[0] === "{" && s[s.length - 1] === "}") ||
         (s[0] === "[" && s[s.length - 1] === "]");
}

export function isWhitelisted(str: string, chars: string[]): boolean {
  if (typeof str !== "string") return false;
  for (let i = 0; i < str.length; i++) {
    let found = false;
    for (let j = 0; j < chars.length; j++) {
      if (str[i] === chars[j]) { found = true; break; }
    }
    if (!found) return false;
  }
  return true;
}
