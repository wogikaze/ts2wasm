#!/usr/bin/env python3
"""Decompose Resolver's 48 flat fields into grouped substructs.

Phase 1: Define group structs, update struct definition, update new()/with_params().
Phase 2: Replace self.field → self.group.field in all method bodies.

Run from repo root:
  python scripts/refactor/decompose-resolver.py

Run with --dry-run to preview changes.
"""

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).parent.parent.parent
MOD_RS = REPO_ROOT / "crates" / "ir" / "src" / "lowered" / "resolver" / "mod.rs"
EXPR_RS = REPO_ROOT / "crates" / "ir" / "src" / "lowered" / "resolver" / "expr.rs"
EXTRA_RS = REPO_ROOT / "crates" / "ir" / "src" / "lowered" / "resolver" / "extra.rs"

FIELDS = {
    "symbols": [
        "function_ids",
        "function_signatures",
    ],
    "locals": [
        "scopes",
        "next_local_id",
        "locals",
        "param_locals",
    ],
    "functions": [
        "function_captures",
        "function_mutable_captures",
        "class_method_captures",
        "class_method_mutable_captures",
        "next_func_id",
        "generated_functions",
    ],
    "captures": [
        "env_cell_names",
        "env_cell_locals",
        "heap_closure_names",
        "heap_closure_locals",
    ],
    "classes": [
        "class_constructor_ids",
        "class_method_ids",
        "class_static_method_ids",
        "class_parents",
        "class_private_fields",
        "class_static_private_fields",
        "local_classes",
        "object_function_props",
        "current_class",
        "in_constructor",
    ],
    "modules": [
        "module_ids",
        "modules",
    ],
    "facts": [
        "arrow_locals",
        "nullish_locals",
        "regexp_literal_locals",
        "invalid_date_locals",
        "bigint_locals",
        "control_flow_bigint_div_rem_locals",
        "control_flow_mixed_bigint_locals",
        "array_locals",
        "static_array_slots",
        "symbol_iterator_object_locals",
        "static_object_literal_locals",
        "static_object_literal_alias_sources",
        "static_function_array_like_locals",
        "string_literal_locals",
        "native_set_add_locals",
        "generator_function_names",
    ],
}

# Build reverse map: field → group
FIELD_TO_GROUP = {}
for group, names in FIELDS.items():
    for name in names:
        assert name not in FIELD_TO_GROUP, f"Duplicate field: {name}"
        FIELD_TO_GROUP[name] = group

# All field names as a set
ALL_FIELDS = set(FIELD_TO_GROUP.keys())

def build_group_structs_indent():
    """Generate the group struct definitions indented to the right level."""
    lines = []
    for group in FIELDS:
        lines.append(f"struct {group.title()} {{")
        for field in FIELDS[group]:
            # Types will need to be filled in; we leave a placeholder
            lines.append(f"    // TODO: {field} type")
        lines.append("}")
        lines.append("")
    return "\n".join(lines)

def gen_group_structs():
    """Generate typed group struct definitions based on original Resolver field types."""
    # Read mod.rs to extract original field types
    text = MOD_RS.read_text()

    # Extract lines between "pub(super) struct Resolver<'a> {" and the closing "}"
    struct_start = text.find("pub(super) struct Resolver<'a> {")
    brace_depth = 0
    struct_end = struct_start
    for i in range(struct_start, len(text)):
        if text[i] == '{':
            brace_depth += 1
        elif text[i] == '}':
            brace_depth -= 1
            if brace_depth == 0:
                struct_end = i + 1
                break

    struct_body = text[struct_start:struct_end]

    # Extract field types
    field_types = {}
    for line in struct_body.split('\n'):
        line = line.strip()
        # Match patterns like: "    function_ids: &'a HashMap<String, FuncId>,"
        # or "    pub(crate) locals: Vec<LocalId>,"
        m = re.match(r'(pub\(crate\)\s+)?(\w+)\s*:\s*(.+),?$', line)
        if m:
            name = m.group(2)
            typ = m.group(3).strip().rstrip(',')
            field_types[name] = typ

    # Generate struct definitions
    out = []
    for group in FIELDS:
        fields = []
        for field_name in FIELDS[group]:
            typ = field_types.get(field_name, "/* TODO: type */")
            vis = "pub(crate) " if field_name in ("locals", "next_func_id", "generated_functions", "modules") else ""
            fields.append(f"    {vis}{field_name}: {typ},")

        group_title = group.title()
        out.append(f"struct {group_title} '{'a'} {{")
        out.extend(fields)
        out.append("}")
        out.append("")

    return "\n".join(out)

def replace_field_access(text):
    """Replace self.field_name and resolver.field_name with self/solver.group.field_name in method bodies."""

    lines = text.split('\n')
    result = []

    for line in lines:
        # Skip comments and non-code lines
        stripped = line.strip()
        if stripped.startswith('//') or stripped.startswith('#') or stripped.startswith('/*'):
            result.append(line)
            continue

        # Replace self.field_name and resolver.field_name
        new_line = re.sub(
            r'\b(self|resolver)\.(\w+)',
            lambda m: f"{m.group(1)}.{FIELD_TO_GROUP[m.group(2)]}.{m.group(2)}"
            if m.group(2) in ALL_FIELDS else m.group(0),
            line
        )
        result.append(new_line)

    return '\n'.join(result)


def transform_text(text, file_path):
    """Apply all transformations to the text."""

    # Phase 2: Replace self.field → self.group.field in method bodies
    text = replace_field_access(text)

    # Phase 1: Update struct definition
    if file_path == MOD_RS:
        text = transform_struct(text)

    return text


def transform_struct(text):
    """Update Resolver struct definition and constructors."""

    # 1. Replace the flat struct fields with grouped struct fields
    # Find the struct body
    struct_start_match = re.search(r'pub\(super\) struct Resolver<\'a> \{', text)
    if not struct_start_match:
        print("ERROR: Could not find Resolver struct definition", file=sys.stderr)
        return text

    struct_start = struct_start_match.start()

    # Find the matching closing brace
    brace_depth = 0
    struct_end = struct_start
    for i in range(struct_start, len(text)):
        if text[i] == '{':
            brace_depth += 1
        elif text[i] == '}':
            brace_depth -= 1
            if brace_depth == 0:
                struct_end = i + 1
                break

    old_struct = text[struct_start:struct_end]

    # Generate new struct body with grouped fields
    group_items = []
    for group in FIELDS:
        group_name = group.title()
        group_vis = "pub(super) " if group == "functions" else "    "
        group_items.append(f"{group_vis}{group}: {group_name}<'a>,")

    new_struct = f"pub(super) struct Resolver<'a> {{\n" + "\n".join(f"    {item}" for item in group_items) + "\n}\n"

    text = text[:struct_start] + new_struct + text[struct_end:]

    # 2. Update new() and with_params() constructors to build group structs
    # These are complex manual edits best done separately.
    # For now, we just mark them

    return text


def dry_run_report(text, path):
    """Count replacements that would be made without actually modifying."""
    lines = text.split('\n')
    changes = []
    for i, line in enumerate(lines, 1):
        matches = re.finditer(r'\bself\.(\w+)', line)
        for m in matches:
            field = m.group(1)
            if field in ALL_FIELDS:
                changes.append((i, field, FIELD_TO_GROUP[field]))

    return changes


def main():
    dry_run = "--dry-run" in sys.argv
    skip_mod = "--skip-mod" in sys.argv

    paths = [p for p in [MOD_RS, EXPR_RS, EXTRA_RS] if not (p == MOD_RS and skip_mod)]

    for path in paths:
        if not path.exists():
            print(f"SKIP {path} (not found)", file=sys.stderr)
            continue

        text = path.read_text()

        if not dry_run:
            print(f"\nProcessing {path.relative_to(REPO_ROOT)}...")
            text = replace_field_access(text)
            path.write_text(text)
            print(f"  Updated.")
        else:
            changes = dry_run_report(text, path)
            if changes:
                print(f"\n=== {path.relative_to(REPO_ROOT)} ===")
                print(f"  {len(changes)} field accesses to update:")
                by_group = {}
                for line_no, field, group in changes:
                    by_group.setdefault(group, []).append((line_no, field))
                for group, items in sorted(by_group.items()):
                    print(f"\n  {group}:")
                    for line_no, field in sorted(items):
                        prefix = "self." if path == MOD_RS else "self./resolver."
                        print(f"    L{line_no}: {prefix}{field}")

    print(f"\n{'Dry run' if dry_run else 'Modifications'} complete.")


if __name__ == "__main__":
    main()
