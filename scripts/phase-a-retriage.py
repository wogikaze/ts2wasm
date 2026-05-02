#!/usr/bin/env python3
"""Phase A: Re-triage audit for issue #5000 child issues.

Reads all open issues that depend on [5000], classifies them by title keywords,
and updates their area/depends_on accordingly. Issues that remain under 5000
are true parser-syntax issues.
"""

import os
import re
import shutil

ISSUES_DIR = "issues/open"

# Classification rules: (keywords, new_area, new_depends_on, meta_label)
# Order matters: first match wins
CLASSIFICATION_RULES = [
    # === SEMANTIC (meta 5001) — type checking, narrowing, inference ===
    (r"(?i)(narrow(ing)?|control.?flow|flow|discriminat|exhaustive)", "frontend/semantics", [5001], "type-narrowing/flow"),
    (r"(?i)(type.?parameter|generic|type.?arg|type.?infer)", "frontend/semantics", [5001], "generics"),
    (r"(?i)(class.*(extends|inherit|derived|base|implement))|(inherit|derived|base).*class", "frontend/semantics", [5001], "class-hierarchy"),
    (r"(?i)(assign.*compat|compat.*assign|contextual.?type)", "frontend/semantics", [5001], "compatibility"),
    (r"(?i)(constraint)", "frontend/semantics", [5001], "constraints"),
    (r"(?i)(private|protected|public|readonly|access.?modif)", "frontend/semantics", [5001], "access-modifiers"),
    (r"(?i)(overload)", "frontend/semantics", [5001], "overload-resolution"),
    (r"(?i)(spread|rest.*param)", "frontend/semantics", [5001], "spread-rest"),
    (r"(?i)(getter|setter|accessor)", "frontend/semantics", [5001], "accessors"),
    (r"(?i)(mapped.?type|conditional.?type|template.?literal.?type|keyof|indexed.?access)", "frontend/semantics", [5001], "advanced-types"),
    (r"(?i)(weak.?type|non.?null|definite.?assign)", "frontend/semantics", [5001], "type-assertions"),
    (r"(?i)(recursive.?type|recursive.?infer)", "frontend/semantics", [5001], "recursive-types"),
    (r"(?i)(constructor.*(overload|return|param))", "frontend/semantics", [5001], "constructor"),
    (r"(?i)(excess.?prop|switch.*exhaustive|no.?fallthrough)", "frontend/semantics", [5001], "exhaustiveness"),
    (r"(?i)(type.?guard|assertion.*func)", "frontend/semantics", [5001], "type-guards"),
    (r"(?i)(literal.?type|template.?literal|union.?type|intersection)", "frontend/semantics", [5001], "type-expressions"),
    (r"(?i)(binary.*arithmetic|arithmetic.*binary)", "frontend/semantics", [5001], "type-operators"),
    (r"(?i)(this.*type|polymorphic)", "frontend/semantics", [5001], "this-types"),
    # === RESOLVER (meta 5005) — names, scope, modules ===
    (r"(?i)(collision|shadow|duplicate|conflict|argument.*name)", "frontend/resolver", [5005], "name-collisions"),
    (r"(?i)(unused|no.?implicit.?any|reachab|unreachable)", "frontend/resolver", [5005], "unused-reachability"),
    (r"(?i)(export.*assign|import.*non.?export|re.?export)", "frontend/resolver", [5005], "import-export"),
    (r"(?i)(module.*resolv|path.*mapping|symlink)", "frontend/resolver", [5005], "module-resolution"),
    (r"(?i)(augmented.?type|merged.*decl|decl.*merg)", "frontend/resolver", [5005], "declaration-merging"),
    (r"(?i)(ambient.*(module|namespace))|(module|namespace).*decl", "frontend/resolver", [5005], "ambient-modules"),
    # === RUNTIME (meta 5004) — builtins, runtime behavior ===
    (r"(?i)(promise|iterable|iterator|generator|yield|async)", "runtime/builtins", [5004], "async-iterable"),
    (r"(?i)(bigint)", "runtime/builtins", [5004], "bigint"),
    (r"(?i)(array.*(filter|from|concat|reduce|find|every|some|flat|fill|copy))|arraybuffer|typedarray", "runtime/builtins", [5004], "array-builtins"),
    (r"(?i)(lib.*type|lib.*resolv)", "runtime/builtins", [5004], "lib-resolution"),
    (r"(?i)(arguments)", "runtime/builtins", [5004], "arguments-object"),
    (r"(?i)(regexp|regex)", "runtime/builtins", [5004], "regexp"),
    (r"(?i)(string.*(match|replace|search|split|slice|trim|pad|repeat|start|end|locale))", "runtime/builtins", [5004], "string-methods"),
    (r"(?i)(number.*(is|parse|to))|parseint|parsefloat", "runtime/builtins", [5004], "number-methods"),
    (r"(?i)(math\.|math_)", "runtime/builtins", [5004], "math"),
    (r"(?i)(json|date|error|eval|settimeout|clear)", "runtime/builtins", [5004], "builtins"),
    # === ERASURE (stays in 5000, but mark as simple) — TS-only skip constructs ===
    # These are true parser issues but simple erasures
    (r"(?i)(declare.*(enum|class|func|var|let|const|module|namespace|global))", "frontend/syntax", [5000], "erasure-ambient"),
    (r"(?i)(source.?map)", "frontend/syntax", [5000], "source-map"),
]


def classify(title: str) -> tuple:
    """Returns (new_area, new_depends_on, reason) or None if unclassified."""
    for pattern, new_area, new_depends, label in CLASSIFICATION_RULES:
        if re.search(pattern, title):
            return (new_area, new_depends, label)
    return None


def update_issue(filepath: str, new_area: str, new_depends: list) -> bool:
    """Update area and depends_on in issue frontmatter."""
    with open(filepath) as f:
        content = f.read()

    # Update area
    content = re.sub(r'^area: .*$', f'area: {new_area}', content, flags=re.MULTILINE)
    # Update depends_on
    depends_str = "[" + ", ".join(str(d) for d in new_depends) + "]"
    content = re.sub(r'^depends_on: \[.*\]$', f'depends_on: {depends_str}', content, flags=re.MULTILINE)

    with open(filepath, 'w') as f:
        f.write(content)
    return True


def main():
    parser_issues = []
    reclassified = []
    unclassified = []

    for fname in sorted(os.listdir(ISSUES_DIR)):
        if not fname.endswith(".md"):
            continue
        fpath = os.path.join(ISSUES_DIR, fname)

        with open(fpath) as f:
            content = f.read()

        # Only process issues that depend on [5000]
        if not re.search(r'depends_on: \[5000\]', content):
            continue

        # Extract title
        title_match = re.search(r'^title: "(.*)"', content, re.MULTILINE)
        title = title_match.group(1) if title_match else fname

        result = classify(title)
        if result is None:
            unclassified.append((fname, title))
            parser_issues.append(fname)
            continue

        new_area, new_depends, label = result
        if new_depends == [5000]:
            # Stays as parser issue
            parser_issues.append(fname)
            continue

        # Reclassify
        update_issue(fpath, new_area, new_depends)
        reclassified.append((fname, title, new_area, new_depends, label))
        print(f"RECLASSIFIED: {fname}")
        print(f"  Title: {title}")
        print(f"  -> area={new_area}, depends_on={new_depends} ({label})")

    print(f"\n{'='*60}")
    print(f"RESULTS")
    print(f"{'='*60}")
    print(f"Reclassified: {len(reclassified)}")
    print(f"Still parser (#5000): {len(parser_issues)}")
    print(f"Unclassified (needs manual review): {len(unclassified)}")
    print()

    if unclassified:
        print("UNCLASSIFIED (manual review needed):")
        for fname, title in unclassified[:30]:
            print(f"  {fname}: {title}")
        if len(unclassified) > 30:
            print(f"  ... and {len(unclassified) - 30} more")


if __name__ == "__main__":
    main()
