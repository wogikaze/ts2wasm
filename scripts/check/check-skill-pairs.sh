#!/bin/bash
# Check that each skill has both SKILL.md (English) and SKILL-ja.md (Japanese)

SKILLS_DIR="/home/wogikaze/ts2wasm/.agents/skills"
MISSING_ENGLISH=()
MISSING_JAPANESE=()
COMPLETE=()

for skill_dir in "$SKILLS_DIR"/*/; do
    skill_name=$(basename "$skill_dir")
    english="$skill_dir/SKILL.md"
    japanese="$skill_dir/SKILL-ja.md"

    if [ -f "$english" ] && [ -f "$japanese" ]; then
        COMPLETE+=("$skill_name")
    elif [ -f "$english" ]; then
        MISSING_JAPANESE+=("$skill_name")
    elif [ -f "$japanese" ]; then
        MISSING_ENGLISH+=("$skill_name")
    fi
done

echo "=== Skill Language Pair Check ==="
echo ""

if [ ${#COMPLETE[@]} -gt 0 ]; then
    echo "✓ Complete (both EN and JA):"
    for skill in "${COMPLETE[@]}"; do
        echo "  - $skill"
    done
    echo ""
fi

if [ ${#MISSING_JAPANESE[@]} -gt 0 ]; then
    echo "✗ Missing Japanese (SKILL-ja.md):"
    for skill in "${MISSING_JAPANESE[@]}"; do
        echo "  - $skill"
    done
    echo ""
fi

if [ ${#MISSING_ENGLISH[@]} -gt 0 ]; then
    echo "✗ Missing English (SKILL.md):"
    for skill in "${MISSING_ENGLISH[@]}"; do
        echo "  - $skill"
    done
    echo ""
fi

TOTAL=$((${#COMPLETE[@]} + ${#MISSING_JAPANESE[@]} + ${#MISSING_ENGLISH[@]}))
echo "Summary: ${#COMPLETE[@]}/$TOTAL complete, ${#MISSING_JAPANESE[@]} missing JA, ${#MISSING_ENGLISH[@]} missing EN"

# Exit with error if any are missing
if [ ${#MISSING_JAPANESE[@]} -gt 0 ] || [ ${#MISSING_ENGLISH[@]} -gt 0 ]; then
    exit 1
fi
