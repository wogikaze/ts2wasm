#!/usr/bin/env bash
# Shared feature-label classifier for compiler diagnostic output.
#
# Usage:
#   ts2wasm_feature_label <diag-code> <stderr-file> [source-path]
#
# Output is a stable lowercase label suitable for TestRecord tracking and
# coverage aggregation, for example: class, import-export, async.

ts2wasm_feature_label() {
  local diag_code="$1"
  local stderr_file="$2"
  local source_path="${3:-}"
  local text=""
  if [[ -f "$stderr_file" ]]; then
    text="$(grep -F "[$diag_code]" "$stderr_file" | head -n 1 || true)"
    if [[ -z "$text" ]]; then
      text="$(cat "$stderr_file")"
    fi
    text="$(printf '%s' "$text" | tr '[:upper:]' '[:lower:]')"
  fi
  local path_lc
  path_lc="$(printf '%s' "$source_path" | tr '[:upper:]' '[:lower:]')"

  case "$diag_code" in
    BackendIo) echo "backend-io"; return ;;
    InvariantViolation) echo "invariant-violation"; return ;;
    UnresolvedName) echo "name-resolution"; return ;;
    UnresolvedFunction) echo "function-resolution"; return ;;
    DuplicateFunction) echo "duplicate-function"; return ;;
    DuplicateLocal) echo "duplicate-local"; return ;;
    DuplicateParameter) echo "duplicate-parameter"; return ;;
    NumberOutOfRange) echo "number-range"; return ;;
    ArityMismatch) echo "arity"; return ;;
    InvalidTopLevelReturn) echo "top-level-return"; return ;;
  esac

  case "$path_lc" in
    *"/built-ins/date/"*) echo "date"; return ;;
    *"/built-ins/array/"*) echo "array-builtin"; return ;;
    *"/built-ins/function/"*) echo "function"; return ;;
    *"/built-ins/object/"*) echo "object-builtin"; return ;;
    *"/regexp/"*|*"/regular-expressions/"*|*"/built-ins/regexp/"*) echo "regexp-literal"; return ;;
    *"/built-ins/string/"*) echo "string-builtin"; return ;;
    *"/built-ins/escape/"*|*"/built-ins/unescape/"*) echo "legacy-global-builtin"; return ;;
    *"/built-ins/"*) echo "builtin-api"; return ;;
    *"/annexb/language/comments/"*) echo "html-comment"; return ;;
    *"/annexb/language/eval-code/"*) echo "eval"; return ;;
    *"/annexb/language/expressions/logical-assignment/"*) echo "logical-assignment"; return ;;
    *"/annexb/language/expressions/template-literal/legacy-octal-escape-sequence-"*) echo "legacy-octal-escape"; return ;;
    *"/class/"*|*"/class-"*|*"/classes/"*) echo "class"; return ;;
    *"/module/"*|*"/import/"*|*"/export/"*) echo "import-export"; return ;;
    *"/async-"*|*"/async/"*|*"/generators/"*) echo "async"; return ;;
    *"/destructuring/"*) echo "destructuring"; return ;;
    *"/template/"*) echo "template-literal"; return ;;
    *"/arrow-function/"*|*"/arrow/"*) echo "arrow-function"; return ;;
    *"/spread/"*) echo "spread"; return ;;
    *".tsx"*|*"jsx"*) echo "jsx"; return ;;
    *"declarationemit"*|*"declarationmap"*|*"declare"*) echo "declaration-emit"; return ;;
    *"accessor"*) echo "class-accessor"; return ;;
    *"parameterproperty"*) echo "parameter-property"; return ;;
    *"anonymousclass"*|*"anonclass"*|*"unnamedclass"*|*"classfields"*|*"classfield"*) echo "class"; return ;;
    *"alias"*) echo "type-alias"; return ;;
    *"ambient"*) echo "ambient-declaration"; return ;;
    *"amd"*|*"systemmodule"*) echo "module-system-amd"; return ;;
    *"package"*|*"nodemodules"*|*"paths"*|*"resolution"*) echo "module-resolution"; return ;;
    *"processingdiagnostic"*) echo "type-directive-resolution"; return ;;
    *"exportassignment"*|*"import"*|*"export"*|*"module"*) echo "import-export"; return ;;
    *"enum"*) echo "enum"; return ;;
    *"decorator"*) echo "decorator"; return ;;
    *"assertion"*|*"satisfies"*|*"asconst"*) echo "type-assertion"; return ;;
    *"bindingpattern"*|*"destructur"*) echo "destructuring"; return ;;
    *"conditional"*|*"keyof"*|*"infer"*|*"generic"*|*"typepredicate"*) echo "type-system"; return ;;
    *"scope"*) echo "scope-analysis"; return ;;
    *"arguments"*|*"args"*) echo "arguments-object"; return ;;
    *"objectliteral"*|*"object"*) echo "object-literal"; return ;;
    *"jsdoc"*) echo "jsdoc"; return ;;
  esac

  case "$text" in
    *"class "*) echo "class"; return ;;
    *" import "*|*" export "*|*"require("*|*"require(\""*) echo "import-export"; return ;;
    *"regexp"*|*"regular expression"*) echo "regexp-literal"; return ;;
    *"type annotation"*|*"typescript"*|*"interface "*|*" enum "*) echo "type-annotation"; return ;;
    *"reference types"*|*"type directive"*) echo "type-directive-resolution"; return ;;
    *"destructur"*) echo "destructuring"; return ;;
    *" async "*|*"await "*|*"generator"*) echo "async"; return ;;
    *"=>"*|*"arrow"*) echo "arrow-function"; return ;;
    *"template"*) echo "template-literal"; return ;;
    *"spread"*) echo "spread"; return ;;
    *"rest parameter"*|*"rest "*) echo "rest-parameter"; return ;;
    *"default parameter"*|*"default "*) echo "default-parameter"; return ;;
    *"switch"*) echo "switch"; return ;;
    *"while"*|*"do-while"*|*" for "*) echo "loop"; return ;;
    *"break"*|*"continue"*) echo "break-continue"; return ;;
    *"dynamic propert"*|*"computed propert"*|*"property access"*|*"property key"*) echo "property-access"; return ;;
    *"string literal key"*|*"object literal"*) echo "object-literal"; return ;;
    *"non-ascii"*|*"utf-8"*|*"utf8"*) echo "utf8-string"; return ;;
    *"=="*) echo "equality-operator"; return ;;
    *"binary operator"*|*"unary operator"*) echo "operator"; return ;;
    *"try statement"*|*"catch"*|*"finally"*) echo "try-catch"; return ;;
    *"new classname"*|*"new "*) echo "new-expression"; return ;;
    *"super"*) echo "super"; return ;;
    *"method"*) echo "method-call"; return ;;
    *"constructor"*) echo "class"; return ;;
    *"unsupported character"*|*"unterminated"*|*"expected "*|*"invalid number literal"*) echo "parser-syntax"; return ;;
    *"only identifier calls"*) echo "call-expression"; return ;;
    *"nested function"*|*"kind: function"*) echo "function"; return ;;
    *"expression type not yet supported"*) echo "unsupported-expression"; return ;;
    *"console."*|*"process."*|*"readfilesync"*) echo "builtin-api"; return ;;
  esac

  echo "unknown-unsupported"
}
