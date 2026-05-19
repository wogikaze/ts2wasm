#!/usr/bin/env node
"use strict";

/**
 * Transpile a .ts file to CommonJS .js and execute with Node.
 * Returns stdout/stderr and exit code as if the original was runnable JS.
 *
 * Usage:
 *   node scripts/run/ts-node-oracle.js <input.ts>
 *
 * Exit code: 0 on success, 1 on transpile error, 2 on missing input.
 */

const fs = require("fs");
const path = require("path");

let ts;
try {
  ts = require("typescript");
} catch (error) {
  process.stderr.write(`ts-node-oracle: failed to load typescript: ${error.message}\n`);
  process.exit(2);
}

const input = process.argv[2];
if (!input) {
  process.stderr.write("ts-node-oracle: usage: node scripts/run/ts-node-oracle.js <input.ts>\n");
  process.exit(2);
}

const fileName = path.resolve(input);
let source;
try {
  source = fs.readFileSync(fileName, "utf-8");
} catch (error) {
  process.stderr.write(`ts-node-oracle: failed to read ${fileName}: ${error.message}\n`);
  process.exit(2);
}

const result = ts.transpileModule(source, {
  compilerOptions: {
    target: ts.ScriptTarget.ES2020,
    module: ts.ModuleKind.CommonJS,
    strict: false,
    noImplicitUseStrict: true,
    removeComments: true,
  },
});

if (result.diagnostics && result.diagnostics.length > 0) {
  for (const diag of result.diagnostics) {
    const msg = ts.flattenDiagnosticMessageText(diag.messageText, "\n");
    if (diag.file) {
      const pos = diag.file.getLineAndCharacterOfPosition(diag.start);
      process.stderr.write(`${diag.file.fileName}(${pos.line + 1},${pos.character + 1}): ${msg}\n`);
    } else {
      process.stderr.write(`${msg}\n`);
    }
  }
  process.exit(1);
}

const tmpDir = fs.mkdtempSync(path.join(require("os").tmpdir(), "ts2wasm-"));
const jsFile = path.join(tmpDir, "output.js");
try {
  fs.writeFileSync(jsFile, result.outputText);
  const { spawnSync } = require("child_process");
  const child = spawnSync("node", [jsFile], {
    encoding: "utf-8",
    timeout: 5000,
    stdio: ["ignore", "pipe", "pipe"],
  });
  process.stdout.write(child.stdout);
  process.stderr.write(child.stderr);
  process.exit(child.status === null ? 1 : child.status);
} finally {
  try {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  } catch {
    // best-effort cleanup
  }
}
