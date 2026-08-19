---
document_type: adr
adr_id: ADR-016
status: accepted
date: 2026-08-19
version: "1.0"
subsystems_affected: [scanner]
supersedes: null
superseded_by: null
---

# ADR-016: F-SCAN-DOT-ROOT and F-04-b Remediation Ruling

## Context

**F-SCAN-DOT-ROOT** (Phase 3 Pass-2 adversarial finding) claimed that `filter_entry` rejects any '.'-prefixed entry including ROOT, causing `root="."` → entire scan silently empty. The finding was based on a misreading of the `ignore` crate behavior.

**F-04-b** (Phase 3 Pass-2 adversarial finding) identified that the pure-core guard's per-pattern differential probe exercises only 1/8 forbidden patterns under an `.any()` matcher, meaning a corrupted later pattern would never be caught (short-circuit evaluation).

Both findings required operator adjudication for remediation path.

## Decision

### F-SCAN-DOT-ROOT — OPTION A: NO scanner change

The `ignore` crate does NOT apply `filter_entry` to the ROOT entry. Testing confirmed: `test_F_SCAN_DOT_ROOT_dot_prefixed_root_dir_is_scanned` passes because the dot-prefixed root IS scanned (the filter_entry predicate only applies to child entries, not the root directory itself).

**Remediation actions:**
1. **Keep the existing passing test** (`test_F_SCAN_DOT_ROOT_dot_prefixed_root_dir_is_scanned`)
2. **Correct the false test comment** — the comment claimed filter_entry "skip[s] the entry from output while still descending into it", which is incorrect
3. **NO scanner code changes** — no production bug exists; the finding was a false positive

**Rationale:** The test correctly verifies that dot-prefixed roots are scanned. The comment was wrong (misstating the ignore crate's behavior), but the test itself is valid. Changing the scanner code to add a defensive depth-0 exemption would be unnecessary complexity when the existing behavior is correct.

### F-04-b — OPTION A: Accept per-pattern synthetic-positive probe as-is

The per-pattern differential probe now asserts each of the 8 `FORBIDDEN_PATTERNS` individually via synthetic positives. This verifies each pattern pin is live (the probe fails if any pattern is missing/corrupted).

**Residual:** The probe and matcher share the `FORBIDDEN_PATTERNS` constant, so the probe proves each pin is live but does NOT prove that the pattern set is canonically correct (no independent verification that all 8 patterns are the correct set). This residual is an operator strengthening question, not a gate failure.

**Rationale:** The per-pattern synthetic-positive approach is sufficient for CI usage. The residual (shared constant means independent verification is missing) is acceptable because:
- The pattern set is defined in one place (`FORBIDDEN_PATTERNS`)
- Changes to the pattern set would require a separate ADR
- The probe would fail if any pattern is missing or corrupted

## Consequences

### Positive
- F-SCAN-DOT-ROOT finding is correctly identified as false positive; no scanner change needed
- F-04-b remediation provides confidence each pattern pin is live
- No unnecessary complexity added to scanner code
- Residual documented for future reference

### Negative / Trade-offs
- F-04-b residual means no independent verification that the 8-pattern set is complete (but this is a design-level concern, not a test concern)

## Status as of 2026-08-19

Accepted. Remediation fix-wave executed at HEAD 4820ead:
- F-SCAN-DOT-ROOT: test comment corrected, no scanner change (D-016 opt A)
- F-04-b: per-pattern synthetic positives implemented (D-012 stands, D-016 opt A confirms)

## Alternatives Considered

### F-SCAN-DOT-ROOT
- **OPTION B (rejected):** Add defensive depth-0 root exemption anyway. Rejected because no production bug exists; the ignore crate correctly does not apply filter_entry to ROOT.

### F-04-b
- **OPTION B (rejected):** Require independent pattern set verification (different source for expected patterns). Rejected because the pattern set is defined in one canonical place; independent verification would be redundant complexity.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.0 | 2026-08-19 | operator | Initial recording of F-SCAN-DOT-ROOT and F-04-b remediation rulings |

## Source / Origin

- F-SCAN-DOT-ROOT: Phase 3 Pass-2 adversarial finding; premise (dot-root -> empty scan) disproven by orchestrator execution
- F-04-b: Phase 3 Pass-2 adversarial finding; residual documented as operator strengthening question, not gate failure
- D-014: Operator ruling on F-SCAN-DOT-ROOT re-adjudication (premise DISPROVEN) -> OPTION A
- D-012: Operator ruling on F-04-b residual -> OPTION A (per-pattern synthetic positives)
