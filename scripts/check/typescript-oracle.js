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
      hints: [],
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
      hints: [],
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
const checker = program.getTypeChecker();
const sourceFile = program.getSourceFile(fileName);

function sourceLocation(node) {
  const file = node.getSourceFile();
  const start = node.getStart(file);
  const position = file.getLineAndCharacterOfPosition(start);
  return {
    file: path.resolve(file.fileName),
    start,
    length: node.getWidth(file),
    line: position.line + 1,
    character: position.character + 1,
  };
}

function typeText(node) {
  return checker.typeToString(checker.getTypeAtLocation(node));
}

function addHint(hints, node, hint) {
  hints.push({
    kind: hint.kind,
    typeText: hint.typeText || typeText(node),
    ...sourceLocation(node),
    ...hint,
  });
}

function typeCandidate(leftType, rightType, resultType) {
  if (leftType === "number" && rightType === "number" && resultType === "number") {
    return "number-add-fast-path";
  }
  if (leftType === "string" || rightType === "string") {
    return "string-concat-fast-path";
  }
  return undefined;
}

function collectHints(root) {
  const hints = [];
  if (!root) {
    return hints;
  }

  function visit(node) {
    if (ts.isParameter(node) && ts.isIdentifier(node.name)) {
      addHint(hints, node.name, {
        kind: "parameter",
        name: node.name.text,
      });
    } else if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name)) {
      addHint(hints, node.name, {
        kind: "binding",
        name: node.name.text,
      });
    } else if (ts.isFunctionDeclaration(node) && node.name) {
      const signature = checker.getSignatureFromDeclaration(node);
      const returnType = signature
        ? checker.typeToString(checker.getReturnTypeOfSignature(signature))
        : typeText(node.name);
      addHint(hints, node.name, {
        kind: "function",
        name: node.name.text,
        typeText: returnType,
      });
    } else if (
      ts.isBinaryExpression(node)
      && node.operatorToken.kind === ts.SyntaxKind.PlusToken
    ) {
      const leftType = typeText(node.left);
      const rightType = typeText(node.right);
      const resultType = typeText(node);
      const candidate = typeCandidate(leftType, rightType, resultType);
      addHint(hints, node, {
        kind: "binary-expression",
        operator: "+",
        leftType,
        rightType,
        typeText: resultType,
        ...(candidate ? { candidate } : {}),
      });
    }

    ts.forEachChild(node, visit);
  }

  visit(root);
  return hints;
}

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
    hints: collectHints(sourceFile),
    typescriptVersion: ts.version,
  },
  0,
);
