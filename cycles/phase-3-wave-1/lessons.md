# Lessons Learned — phase-3-wave-1

## Pass 1 (2026-08-19)

**F-02/F-03: Partial AC coverage and semantic anchoring issues**
- **Lesson:** VP-016 source-of-truth H1 "Ignored Files Have Anchor Tables" and "anchor table still built via Pass 1.5" halves of AC-006/AC-010 are structurally undischargeable in S-1.01 without AnchorIndex/run_scan code
- **Resolution:** DEFERRED to BC-2.08.004 / SS-05 story (D-008)
- **Action Item:** Do NOT re-litigate these findings in future adversarial passes - they are formally deferred

## Pass 2 (2026-08-19)

**F-04-b: Pure-core guard probe exercises only 1/8 forbidden patterns**
- **Lesson:** The differential probe uses an .any() matcher which short-circuits, meaning only the first pattern is ever tested
- **Resolution:** D-012 - Per-pattern differential probe with synthetic positives
- **Residual:** The probe and matcher share the same FORBIDDEN_PATTERNS constant, so it proves each pin is live but not that the pattern set is canonically correct (no independent positives). This is an operator strengthening question, not a gate failure.

**F-VP017: Proptest not invoked**
- **Lesson:** Frozen spec states `proof_method=integration` but implementation used hand cases
- **Resolution:** D-013 - Write actual proptest (bounded depth + symlink cycle)

## Pass 3 (2026-08-19)

**F-P3-01: FORBIDDEN_PATTERNS entry "rand::rng" is subsumed under "rand::"**
- **Lesson:** The .any(contains) matcher means "rand::rng" is a strict superstring of "rand::" and is covered by the more general pattern
- **Resolution:** D-012 - Drop the subsumed entry, correct line-97 claim to "7 pins individually live"

**F-P3-02: Oracle robustness - no empty-results check**
- **Lesson:** Fixture .ok() methods mask setup failures silently
- **Resolution:** D-017 - Add prop_assert!(!results.is_empty()) after scan, change .ok() to .expect(...)

## Pass 4 (2026-08-19)

**F-P4-01: Pure-core guard passes GREEN on emptied FORBIDDEN_PATTERNS**
- **Lesson:** With an empty FORBIDDEN_PATTERNS set, the .any(contains) loop always returns false (no items to iterate), so the test passes GREEN on an empty set - this is the "green-on-emptied-input" vacuity class
- **Resolution:** D-018 - Add fail-closed assert!(!FORBIDDEN_PATTERNS.is_empty()) + runtime pin-probe positive-coverage count
- **Verification:** Red-on-empty test (emptied const) → exit 101, fail-closed assert fires. Real 7-pin set → exit 0, runtime emit "PURE-CORE-GUARD: Check passed: 7 patterns probed, 2 files validated"

**Residuals (non-blocking):**
- Comment "independent" over-claim
- scanner.rs:31 terse-comment clarity
