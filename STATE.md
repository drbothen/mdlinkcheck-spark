---
document_type: pipeline-state
level: ops
version: "3.9"
status: draft
producer: state-manager
timestamp: 2026-08-19T14:24:32Z
phase: phase-3
inputs: []
input-hash: "[live-state]"
traces_to: ""
project: mdlinkcheck
mode: greenfield
current_step: "Phase 3 wave 1 — S-1.01 F-01+F-04 fix pair COMPLETE + independently gate-verified (build/fmt/clippy -Dwarnings/nextest all exit 0; 59/59). Convergence 0/3. NEXT: adversarial convergence Pass 2 (fresh-context different-model), F-02/F-03 injected as ADJUDICATED-DEFERRED non-findings, verify F-01/F-04."
current_cycle: phase-3-wave-1
dtu_required: false
---

<!--
  STATE.md SIZE BUDGET:
  Soft target: ≤200 lines; hard cap: 500 lines.
  Historical content belongs in cycle files, NOT here.
  Run /vsdd-factory:compact-state if this file grows past 200 lines.
-->

# Pipeline State: mdlinkcheck

## Project Metadata

| Field | Value |
|-------|-------|
| **Product** | mdlinkcheck |
| **Repository** | /Users/jmagady/Dev/mdlinkcheck-spark |
| **Mode** | greenfield |
| **Language** | Rust (MSRV 1.85, toolchain pinned 1.97.0) |
| **Product Type** | CLI (no UI) |
| **Started** | 2026-08-18 (Phase 3 start from ratified spec package) |
| **Last Updated** | 2026-08-19 — S-1.01 F-01+F-04 fix pair COMPLETE + independently gate-verified (build/fmt/clippy -Dwarnings/nextest all exit 0; 59/59) |
| **Current Phase** | phase-3 |
| **Current Step** | Phase 3 wave 1 — S-1.01 F-01+F-04 fix pair COMPLETE + independently gate-verified (build/fmt/clippy -Dwarnings/nextest all exit 0; 59/59). Convergence 0/3. NEXT: adversarial convergence Pass 2 (fresh-context different-model), F-02/F-03 injected as ADJUDICATED-DEFERRED non-findings, verify F-01/F-04. |

## Phase Progress

| Phase | Status | Started | Completed | Gate | Finding Progression |
|-------|--------|---------|-----------|------|---------------------|
| pre-1: Planning | completed | 2026-08-05 | 2026-08-05 | HUMAN: market-intel-review + intake-approval | |
| 0: Codebase Ingestion | not-applicable (greenfield) | | | | |
| 1: Spec Crystallization | completed | 2026-08-05 | 2026-08-10 | HUMAN: RATIFIED with closed-world remediation completed | |
| 1d: Adversarial Spec Review | completed | 2026-08-05 | 2026-08-10 | HUMAN: ratified with condition; remediation executed and verified | |
| 2: Story Decomposition | completed | 2026-08-10 | 2026-08-10 | HUMAN: ratified 6/6 | 24 stories / 7 epics / 7 waves; holdout scenarios seeded per boundary policy |
| 3: TDD Implementation | in-progress | 2026-08-18 | | wave gates: full suite + adversarial review of wave diff + holdout eval; HUMAN-ratified | 0/3 (Pass 1 ADJUDICATED; fix pair F-01+F-04 complete + verified; Pass 2 pending) |
| 4: Holdout Evaluation | not-started | | | | |
| 5: Adversarial Refinement | not-started | | | | |
| 6: Formal Hardening | not-started | | | | |
| 7: Convergence | not-started | | | | |

## Current Phase Steps

| Step | Status | Notes |
|------|--------|-------|
| Worktree/branch | DONE | worktree /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01, branch feature/S-1.01-workspace-scaffold-and-core-discovery, based on develop f81f412 |
| Stub scaffold | DONE + verified | cargo build --locked green; Red Gate 4 todo!() bodies; mdlinkcheck-core purity-clean |
| Dependency pins | DONE + verified | clap="=4.6.5", unicode-normalization="=0.1.24", proptest="~1.6" |
| ureq removal | DONE + verified | Removed unused ureq="3.3.0" + TLS subtree (282 lock lines) |
| Failing tests (Red Gate) | DONE + VERIFIED | 27/27 scanner tests fail with todo!() panic; control 25/25 core type tests pass |
| Implementer TDD-to-green | DONE + verified | ignore-crate-native rewrite committed at 41b05d8; suite green (52/52 pass) |
| Resume integrity check | DONE + verified | HEAD 41b05d831e1e2aa1423cd5734edd44e2923e5020 matches prior checkpoint; working tree clean; cargo nextest run --locked: 52/52 passed |
| S-1.01 fix-wave | COMPLETE + VERIFIED | HEAD 2859e03; 59/59 passed; all CI-equivalent gates GREEN (build/fmt/clippy -Dwarnings/nextest all exit 0); commits: 3b705eb (F-01/D-010 genuine oracle + F-02/F-03/D-008 comment corrections + fmt), 710d09b (F-04/D-009 pure-core guard + POL-11), 2859e03 (F-01 rustfmt clean) |
| Adversarial convergence pass-1 | COMPLETE + VERIFIED | Fix pair F-01+F-04 complete + independently verified (build/fmt/clippy -Dwarnings/nextest all exit 0; 59/59). F-02/F-03 ADJUDICATED-DEFERRED (D-008). Convergence 0/3; NEXT: Pass 2 (fresh-context adversary, F-02/F-03 injected as non-findings). |

## Convergence Status

Passes validly completed: 0; consecutive clean passes: 0 of 3 required; operator-ruled on Pass-1 escalation (F-01..F-04, all MEDIUM). F-01/F-04 RESOLVED + VERIFIED; F-02/F-03 ADJUDICATED-DEFERRED (D-008).

## Fix Wave Ledger

| Finding | Operator | Status | Notes |
|---------|----------|--------|-------|
| F1 (H3: scanner.rs:52-54 false comment on file-symlinks) | implementer | RESOLVED | Comment corrected; tech-debt deferral recorded per D-007 |
| F2a (H1 dot-ancestor empties scan; BC-2.01.001 PC1) | implementer | RESOLVED | filter_entry fixed to skip dot-DIRECTORIES only; .hidden(false) added |
| F2b (H2/H4 dot-files; operator ruling INCLUDE) | implementer | RESOLVED | Dot-files now included; dot-dir combined test passes |
| F3 (VP-017 real cycle + termination; adversary F-01) | test-writer | RESOLVED | Rewritten in 076c12c, passes; cosmetic unused `results` removed |
| F4 (AC-002 dedup; H5) | test-writer | REOPENED | HashSet-un |
| F5 (AC-008 CLI surface rejects --hidden; H6; D-011) | test-writer | RESOLVED | cli lib module created; real assertion instead of fake panic! |
| F6 (nested .gitignore PC2; BC-2.01.003 PC2; H7) | test-writer | RESOLVED | Real nested .gitignore test (subdir excludes drop.md, keeps keep.md) |
| CLIPPY-01 | implementer | RESOLVED | 41 findings resolved: collapsible_if, 2 unused imports, dead helper, 31 non_snake_case traceability-name, needless borrow, len>=1; fmt test files now green |
| F-01 MEDIUM [content-defect] | test-soundness | ADJUDICATED + RESOLVED + VERIFIED | test_BC_2_01_001_no_duplicate_in_scan_set tautological replaced with GENUINE in-scope assertion (D-010) + BC-2.01.006 vacuity note. Falsifiability PROVEN (phantom file → assert FAIL left:2 right:3). Commit 3b705eb (+ fmt 2859e03). |
| F-02 MEDIUM [content-defect] | partial AC coverage | ADJUDICATED-DEFERRED | AC-006 (story:103-107) and AC-010 (story:124-129) each assert two postconditions; the "anchor table still built via Pass 1.5" half is structurally undischargeable in S-1.01 (no AnchorIndex/run_scan/Pass 1.5 code). Tests verify only the "not in scan set" half. Operator ruling (D-008): DEFERRED to BC-2.08.004 / SS-05 story (NOT blocking for Pass 2). Test comments corrected in 3b705eb to state the deferral honestly. |
| F-03 MEDIUM [content-defect] | semantic anchoring | ADJUDICATED-DEFERRED | POLICY 4 FAIL: VP-016 source-of-truth H1 "Ignored Files Have Anchor Tables", 5 fixtures require run_scan/Pass 1.5/AnchorIndex. Story:71-72 mischaracterizes VP-016 as ".gitignore exclusion — files never in scan set". test_VP_016_* verifies only the exclusion premise, not anchor-target-resolution. VP-016's module (anchor_table) != story target_module (scanner). Operator ruling (D-008): DEFERRED to BC-2.08.004 / SS-05 story (NOT blocking for Pass 2). Test comments corrected in 3b705eb to state the deferral honestly (removed VP-016 over-claim). |
| F-04 MEDIUM [process-gap] | pure-core enforcement absent | ADJUDICATED + RESOLVED + VERIFIED | Mechanical pure-core I/O guard at crates/mdlinkcheck-core/tests/pure_core_guard.rs (D-009). POL-11 positive-coverage (reached-count=2, types.rs+lib.rs, asserted nonzero + closed enumeration). File-scan falsifiability PROVEN (injected use std::fs → exit 101; reverted → exit 0). Commit 710d09b (amended fmt-clean). |

## Decisions Log

| ID | Date | Decision |
|----|------|----------|
| D-001 | 2026-08-18 | Phase 3 initialized from the ratified spec package at develop f81f412. Engagement scope: waves 1→4 per sprint-state.yaml; wave gates HUMAN-ratified; merges/verdicts/PR creation HUMAN-executed. |
| D-002 | 2026-08-18 | Operator ruling: pin clap="=4.6.5", unicode-normalization="=0.1.24", proptest="~1.6" (resolved 1.6.0) to match ratified dependency-graph.md verified-version table. Verified in Cargo.lock; only proptest's dev subtree (rand 0.8 family, getrandom, lazy_static) shifted; 8 production deps unchanged. Commit 0025791. |
| D-003 | 2026-08-18 | Orchestrator decision (process): removed out-of-scope `ureq="3.3.0"` from S-1.01 binary manifest (unused in S-1.01 source; not in story Library Requirements; deferred to E-5 HTTP stories, first use S-5.02, to be pinned =3.3.0). Pruned entire rustls/ring/webpki TLS subtree (282 lock lines). Commit 5161fcb. |
| D-004 | 2026-08-18 | Orchestrator decision (process): accepted test-writer's addition of crates/mdlinkcheck/src/lib.rs (`pub mod scanner;`) + scanner.rs `use std::path::PathBuf;` as the minimal, benign enabler for the story-mandated integration tests (binary crate becomes bin+lib hybrid; no behavior change, no todo!() removed, architecture does not forbid a lib target). |
| D-005 | 2026-08-19 | Operator ruling: fix tests to spec-correct oracles for the two gitignore over-exclusion tests (VP-016 and mixed-scenario); test-file-only change. Commit 3e1f253. |
| D-006 | 2026-08-19 | Process: expose CliArgs via the lib crate (crates/mdlinkcheck/src/cli.rs, pub) so AC-008/F5 can assert the CLI surface has no --hidden; consistent with D-004 (bin+lib hybrid). Implementer source work, pending. |
| D-007 | 2026-08-19 | Operator-ruled durable: PC3 file-symlink following DEFERRED to BC-2.01.006; S-1.01 fix = correct the false comment + record tech-debt deferral entry (do NOT implement follow_links for files). |
| D-008 | 2026-08-19 | Operator ruling — F-02/F-03 ACCEPT + DEFER, no spec edit: VP-016 (source_bc BC-2.08.004, module anchor_table) and the "anchor table still built via Pass 1.5" halves of AC-006 and AC-010 are formally DEFERRED to the story implementing run_scan + Pass 1.5 + AnchorIndex (BC-2.08.004 / SS-05 owner). S-1.01 discharges ONLY its in-scope discovery halves (exclusion / not-in-scan-set). This is a DOCUMENTED DEFERRAL — no spec edit. Future adversarial passes MUST treat F-02 and F-03 as ADJUDICATED-DEFERRED (not blocking). The POLICY 4 semantic-anchoring inaccuracy in the S-1.01 story's VP-016 description is a known frozen-spec issue carried as tech-debt (spec-text fix deferred). |
| D-009 | 2026-08-19 | Operator ruling — F-04: Add a cheap MECHANICAL pure-core I/O guard NOW, test/CI-only, with a POL-11 positive-coverage assertion, asserting crates/mdlinkcheck-core/src contains no std::fs / std::net / std::io::stdout / Instant::now / RNG imports. Correct the enforcement wording in a test/CI comment. The frozen S-1.01 compliance-table wording ("cargo deny rule; Kani harnesses") fix is DEFERRED as tech-debt (no spec edit this cycle). |
| D-010 | 2026-08-19 | Operator ruling — F-01: Test-only honest fix — replace the tautological dedup oracle (returned_len == HashSet(returned).len()) with a GENUINE in-scope assertion comparing the returned set against the independently-known set of .md files the test itself created; add a vacuity note that BC-2.01.001 PC2's multiple-traversal-paths clause is vacuous under follow_links(false) and becomes non-vacuously testable only once symlink-following lands (anchored to BC-2.01.006). No spec edit. |

## Skip Log

| ID | Date | Skipped | Reason |
|----|------|---------|--------|

## Blocking Issues

| ID | Issue | Severity | Blocking Phase | Owner | Resolution |
|----|-------|----------|---------------|-------|------------|
| F-01 | AC-002 dedup tautological — replaced with GENUINE in-scope assertion (D-010) + BC-2.01.006 vacuity note. Falsifiability PROVEN (phantom file → assert FAIL left:2 right:3). | MEDIUM | phase-3 | test-writer | ADJUDICATED + RESOLVED + VERIFIED (D-010, commit 3b705eb + fmt 2859e03). |
| F-02 | AC-006 and AC-010 partial coverage — each asserts two postconditions; "anchor table still built via Pass 1.5" half structurally undischargeable in S-1.01 (no AnchorIndex/run_scan/Pass 1.5 code). Tests verify only the "not in scan set" half. | MEDIUM | phase-3 | test-writer | ADJUDICATED-DEFERRED by operator (D-008); DEFERRED to BC-2.08.004 / SS-05 story (NOT blocking for Pass 2). Test comments corrected in 3b705eb to state the deferral honestly. |
| F-03 | VP-016 semantic anchoring POLICY 4 FAIL — VP-016 source-of-truth H1 "Ignored Files Have Anchor Tables", 5 fixtures require run_scan/Pass 1.5/AnchorIndex. Story:71-72 mischaracterizes VP-016 as ".gitignore exclusion — files never in scan set". test_VP_016_* verifies only the exclusion premise, not anchor-target-resolution. VP-016's module (anchor_table) != story target_module (scanner). | MEDIUM | phase-3 | spec-steward | ADJUDICATED-DEFERRED by operator (D-008); DEFERRED to BC-2.08.004 / SS-05 story (NOT blocking for Pass 2). Test comments corrected in 3b705eb to state the deferral honestly (removed VP-016 over-claim). |
| F-04 | Pure-core enforcement absent — added mechanical pure-core I/O guard at crates/mdlinkcheck-core/tests/pure_core_guard.rs (D-009). POL-11 positive-coverage (reached-count=2, types.rs+lib.rs, asserted nonzero + closed enumeration). File-scan falsifiability PROVEN (injected use std::fs into core/src → exit 101; reverted → exit 0). | MEDIUM | phase-3 | architect | ADJUDICATED + RESOLVED + VERIFIED (D-009, commit 710d09b). |

## Drift Items

- [process-gap] Orchestrator context auto-compacted mid-directive; a checkpoint commit was lost (factory-artifacts remained at its prior tip). Codification follow-up: checkpoint EARLIER and in smaller increments; treat post-compaction summary content as unverified until re-anchored to disk.
- [process-gap] Red Gate verified test redness but not oracle correctness (two gitignore over-exclusion tests were spec-wrong; surfaced only during implementation). Add oracle-correctness spot-check to Red Gate/test-review.
- [process-gap] Red Gate/test-review did not catch a non-exercising "property" test (proptest imported but never invoked; symlink branch gated on never-created paths). Add a gate check that property tests actually invoke a generator and fixture branches are not dead.
- [process-gap] Implementer self-reported FMT_EXIT=0 but test files were committed fmt-dirty; caught only by orchestrator's independent execution. Reaffirms reports-are-not-evidence; orchestrator must run the FULL gate (fmt+clippy+tests), not trust subagent gate self-reports.
- [process-gap][recurrence x3] Test files committed fmt-dirty AGAIN this cycle — both 3b705eb (scanner_discovery_tests.rs) and the original F-04 commit (pure_core_guard.rs) failed  as first committed; caught only by the orchestrator's independent gate, fixed in 710d09b (amend) + 2859e03. This is the 3rd recorded occurrence of the 'reports-are-not-evidence: subagent self-reports fmt-clean but commits fmt-dirty' gap. Codification (a per-story-chain pre-commit/CI fmt gate) is DEFERRED for this engagement per operator directive 'no pre-gate factory/tooling work'; carried as tech-debt, target: post-wave-4 factory hardening. Interim mitigation: orchestrator continues to run the full independent gate (fmt+clippy+nextest) on every committed fix, never trusting subagent gate self-reports.

## Session Resume Checkpoint

DISK-VERIFIED STATE (as of 2026-08-19T14:24:32Z):
- Feature branch: feature/S-1.01-workspace-scaffold-and-core-discovery
- HEAD SHA: 2859e03ca7c5c52e2960979fabecb148b1edfc96
- Working tree: clean
- Test command: cargo nextest run --locked --workspace --no-fail-fast
- EXIT code: 0
- Pass/fail counts: 59 passed, 0 fail
- Toolchain verified: cargo 1.97.0, rustc 1.97.0, cargo-nextest 0.9.129

S-1.01 F-01+F-04 fix pair COMPLETE + independently gate-verified (build/fmt/clippy -Dwarnings/nextest all exit 0; 59/59). Convergence 0/3. NEXT: adversarial convergence Pass 2 (fresh-context different-model), F-02/F-03 injected as ADJUDICATED-DEFERRED non-findings, verify F-01/F-04.

## Adversarial Pass 1

- Fresh-context different-model adversary with policies.yaml rubric
- Lens perimeter declared; F1-F7 handed as unverified hints
- Convergence clean-pass streak = 0 of 3
- 4 MEDIUM findings ESCALATED to operator (F-01..F-04)

## NEXT ACTION

Dispatch adversarial convergence Pass 2 for S-1.01 (fresh-context, different-model adversary, policies.yaml rubric). Inject F-02 and F-03 as ADJUDICATED-DEFERRED non-findings (D-008) — must NOT be re-raised as blocking. Instruct the adversary to verify the F-01 genuine-oracle fix (D-010) and the F-04 pure-core guard (D-009) as landed at HEAD 2859e03. 3 consecutive clean passes still required to converge S-1.01 (currently 0/3).

## Concurrent Cycles

| Cycle | Type | Status |
|-------|------|--------|
| phase-3-wave-1 | feature | in-progress (S-1.01 F-01+F-04 fix pair COMPLETE + VERIFIED; convergence 0/3; NEXT: Pass 2) |

## Historical Content

| Content | Location |
|---------|----------|
| Burst history | `cycles/phase-3-wave-1/burst-log.md` |
| Convergence trajectory | `cycles/phase-3-wave-1/convergence-trajectory.md` |
| Session checkpoints | `cycles/phase-3-wave-1/session-checkpoints.md` |
| Lessons learned | `cycles/phase-3-wave-1/lessons.md` |
| Resolved blockers | `cycles/phase-3-wave-1/blocking-issues-resolved.md` |
| Cycle manifest | `cycles/phase-3-wave-1/cycle-manifest.md` |
