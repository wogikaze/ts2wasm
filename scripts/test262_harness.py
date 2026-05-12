#!/usr/bin/env python3
"""Inject minimal assert.js and sta.js when test262 requires them.

Usage:
  python scripts/test262_harness.py --self-test
"""

import sys


INLINE_STA_JS = """
function Test262Error(message) {
  this.message = message || "";
}
"""

INLINE_ASSERT_JS = """
var assert = {};
assert.sameValue = function(actual, expected) {
  var same = actual === expected;
  if (!same && typeof actual === "number" && typeof expected === "number") {
    same = actual !== actual && expected !== expected;
  }
  if (!same) {
    throw new Test262Error(" expected same value");
  }
};
"""

HARNESS_SOURCES = {
    "sta.js": INLINE_STA_JS,
    "assert.js": INLINE_ASSERT_JS,
}


def get_harness_sources(includes):
    """Return list of JS source code strings for requested test262 harness includes.

    Args:
        includes: list of harness file names (e.g. ["assert.js", "sta.js"])

    Returns:
        list of JS source code strings for each requested include
    """
    sources = []
    for name in includes:
        if name in HARNESS_SOURCES:
            sources.append(HARNESS_SOURCES[name])
    return sources


def main():
    if len(sys.argv) >= 2 and sys.argv[1] == "--self-test":
        sources = get_harness_sources(list(HARNESS_SOURCES.keys()))
        print(f"Loaded {len(sources)} harness sources:")
        for name, src in HARNESS_SOURCES.items():
            line_count = len(src.strip().split("\n"))
            print(f"  {name}: {line_count} lines")
        sys.exit(0)

    print("Usage: python scripts/test262_harness.py --self-test")
    sys.exit(0)


if __name__ == "__main__":
    main()
