# Full Architectural Analysis

See conversation context from 2026-05-11.

This is the complete 25-section architectural analysis covering:

1. Current coupling points (9 identified)
2. Triple separation principle (phase × semantic domain × capability)
3. Target architecture (14-crate hierarchy)
4. LLM-friendly file size standards (300-1500 LOC)
5. Pipeline/I/O separation for compiler
6. Validated<T> pattern
7. Resolver context decomposition
8. Real module boundaries (replace include!)
9. Runtime catalog domain splitting
10. RuntimeCall String → typed enum
11. HIR/MIR responsibility separation
12. WAT encoder improvements
13. Diagnostic crate extraction
14. Vertical slice feature checklist
15. Architecture fitness functions (18 checks)
16. Coupling metrics (fan-out, public API, match size, etc.)
17. Change reason separation principle
18. Object semantics kernel and completion records
19. Multi-layer test strategy
20. Refactoring methodology (Branch by abstraction, Strangler fig, etc.)
21. Priority-ordered 10-item action plan
22. Design slogans
23. Target "ideal work unit" file list
24. Missing root Cargo.toml/current-state.md note
25. Shortest path to improvement (8 steps)
