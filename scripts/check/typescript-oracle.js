#!/usr/bin/env node
"use strict";

const path = require("path");

function jsonAndExit(payload, code) {
  process.stdout.write(`${JSON.stringify(payload)}\n`);
  process.exit(code);
}

let ts;
try {
  ts = require("typescript");
} catch (error) {
  jsonAndExit(
    {
      ok: false,
      error: `failed to load TypeScript compiler API: ${error.message}`,
      diagnostics: [],
    },
    2,
  );
}

const input = process.argv[2];
if (!input) {
  jsonAndExit(
    {
      ok: false,
      error: "usage: node scripts/check/typescript-oracle.js <input.ts>",
      diagnostics: [],
      typescriptVersion: ts.version,
    },
    2,
  );
}

const fileName = path.resolve(input);
const options = {
  allowJs: false,
  checkJs: false,
  module: ts.ModuleKind.CommonJS,
  noEmit: true,
  noImplicitAny: false,
  skipLibCheck: true,
  strict: true,
  target: ts.ScriptTarget.ES2020,
};

const program = ts.createProgram([fileName], options);
const diagnostics = ts.getPreEmitDiagnostics(program).map((diagnostic) => {
  const message = ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n");
  const item = {
    code: diagnostic.code,
    category: ts.DiagnosticCategory[diagnostic.category],
    message,
  };

  if (diagnostic.file) {
    item.file = path.resolve(diagnostic.file.fileName);
    if (typeof diagnostic.start === "number") {
      const position = diagnostic.file.getLineAndCharacterOfPosition(diagnostic.start);
      item.start = diagnostic.start;
      item.length = diagnostic.length || 0;
      item.line = position.line + 1;
      item.character = position.character + 1;
    }
  }

  return item;
});

jsonAndExit(
  {
    ok: diagnostics.length === 0,
    diagnostics,
    typescriptVersion: ts.version,
  },
  0,
);
