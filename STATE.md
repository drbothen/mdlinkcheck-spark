---
document_type: pipeline-state
level: ops
version: "3.20"
status: draft
producer: state-manager
timestamp: 2026-08-20T23:25:00Z
phase: phase-3
inputs: []
input-hash: "[live-state]"
traces_to: ""
project: mdlinkcheck
mode: greenfield
current_step: "Pass 12 CLEAN @45c30f6 (D-028 comment-only fix L616/L655 applied + verified 61/61); convergence streak 1 of 3; PR packaging BLOCKED until 3/3; clean-stop."
current_cycle: phase-3-wave-1
dtu_required: false
---

# Pipeline State: mdlinkcheck

## Project Metadata

| Field | Value |
|-------|-------|
| **Product** | mdlinkcheck |
| **Repository** | /Users/jmagady/Dev/mdlinkcheck-spark |
| **Mode** | greenfield |
| **Language** | Rust (MSRV 1.85, toolchain pinned 1.97.0) |
| **Product Type** | CLI (no UI) |
| **Started** | 2026-08-05 |
| **Last Updated** | 2026-08-20 - S-1.01 Adversarial Pass 12 CLEAN @45c30f6 (D-028); convergence streak 1 of 3; PR packaging BLOCKED until 3/3.
| **Current Phase** | phase-3 |
| **Current Step** | Pass 12 CLEAN @45c30f6 (D-028 comment-only fix L616/L655 applied + verified 61/61); convergence streak 1 of 3; PR packaging BLOCKED until 3/3; clean-stop.

## Phase Progress

| Phase | Status | Started | Completed | Gate | Finding Progression |
|-------|--------|---------|-----------|------|---------------------|
| pre-1: Planning | completed | 2026-08-05 | 2026-08-05 | HUMAN: market-intel-review + intake-approval | |
| 0: Codebase Ingestion | not-applicable (greenfield) | | | | |
| 1: Spec Crystallization | completed | 2026-08-05 | 2026-08-10 | HUMAN: ratified with closed-world remediation completed | |
| 1d: Adversarial Spec Review | completed | 2026-08-05 | 2026-08-10 | HUMAN: ratified with condition; remediation executed and verified | |
| 2: Story Decomposition | completed | 2026-08-10 | 2026-08-10 | HUMAN: ratified 6/6 | 24 stories / 7 epics / 7 waves; holdout scenarios seeded per boundary policy |
3: TDD Implementation | complete | 2026-08-18 | 2026-08-20 | wave gates: full suite + adversarial review of wave diff + holdout eval; HUMAN-ratified; Adversarial Pass 12 CLEAN @45c30f6 (D-028); convergence streak 1 of 3 | 29→24→21→7→4→3→2→0→0→1

## Current Phase Steps

| Step | Status | Notes |
|------|--------|-------|
| Worktree/branch | DONE | worktree .worktrees/S-1.01, branch feature/S-1.01-workspace-scaffold-and-core-discovery, based on develop f81f412 |
| Stub scaffold | DONE + verified | cargo build --locked green; Red Gate 4 todo!() bodies; mdlinkcheck-core purity-clean |
| Dependency pins | DONE + verified | clap="=4.6.5", unicode-normalization="==0.1.24", proptest="~1.6" |
| ureq removal | DONE + verified | Removed unused ureq="3.3.0" + TLS subtree (282 lock lines) |
| Failing tests (Red Gate) | DONE + VERIFIED | 27/27 scanner tests fail with todo!() panic; control 25/25 core type tests pass |
| Implementer TDD-to-green | DONE + verified | ignore-crate-native rewrite committed at 41b05d8; suite green (52/52 pass) |
| S-1.01 fix-wave | COMPLETE + VERIFIED | HEAD 2859e03; 59/59 passed; all CI-equivalent gates GREEN (build/fmt/clippy -Dwarnings/nextest all exit 0); commits: 3b705eb (F-01/D-010), 710d09b (F-04/D-009), 2859e03 (fmt) |
| Adversarial convergence pass-1 | COMPLETE + VERIFIED | F-02/F-03 ADJUDICATED-DEFERRED (D-008). Convergence 0/3; F-01/F-04 RESOLVED + VERIFIED. NEXT: Pass 2. |
| Adversarial convergence pass-2 | NOT CLEAN | Fresh-context different-model adversary; 4 MEDIUM findings ESCALATED (F-04-a,F-04-b,F-VP017,F-SCAN-DOT-ROOT); SESSION WRAPPED. |
| S-1.01 Pass-2 remediation fix-wave | COMPLETE + VERIFIED | HEAD 4820ead; CI-equiv: build 0/fmt 0/clippy 0/nextest 61/61 passed 0 skipped; all 4 findings dispositioned: F-04-a RESOLVED, F-04-b accepted-residual, F-VP017 fixed, F-SCAN-DOT-ROOT premise-disproven+comment-fixed |
| Adversarial convergence pass-3 | COMPLETE + NOT CLEAN | Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-007/D-016 supplied as ground truth; 4 findings operator-adjudication-pending (F-P3-01 to F-P3-04); convergence streak 0/3; REMEDIATED at FEAT_SHA 46101ae: 2 MEDIUM (F-P3-01, F-P3-02); 2 did NOT survive re-derivation as material |
| Pass-3 fix wave (F-P3-01, F-P3-02) | COMPLETE+VERIFIED | Gate C-#7: 61/61 passed, 0 skipped; FEAT_SHA 46101ae |
| Adversarial convergence pass-4 | NOT CLEAN | Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-016/D-017 supplied as ground truth; 1 MEDIUM finding (F-P4-01), 2 LOW residuals non-blocking; REMEDIATED at FEAT_SHA f468bd5 (D-018); convergence streak 0/3. |
| Pass-4 fix wave (F-P4-01) | COMPLETE+VERIFIED | Gate C-#7: 61/61 passed, 0 skipped; FEAT_SHA f468bd5 |
| Adversarial convergence pass-5 | NOT CLEAN (fix-wave - remediation, not clean-pass) | Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-018/D-019 supplied as ground truth; 1 finding (F-P5-01) ESCALATED; REMEDIATED at FEAT_SHA 9d1a6bb (D-019); convergence streak 0/3. |
| Pass-5 fix wave (F-P5-01) | COMPLETE+VERIFIED | Comment-only fix; 61/61 passed, 0 skipped; FEAT_SHA 9d1a6bb |
| Adversarial convergence pass-6 | CLEAN (first substantiated clean pass) | Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-019 supplied as ground truth; 0 findings (F-P5-01 comment resolved). Convergence streak 1 of 3. |
| Adversarial convergence pass-7 | NOT CLEAN | Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-019/D-021 supplied as ground truth; 2 findings ESCALATED (F-P7-01, F-P7-02) ESCALATED-PENDING-OPERATOR; convergence streak reset 1→0. REMEDIATED at FEAT_SHA ee89580 (D-023 comment-only); convergence streak 0/3. |
| Pass-7 remediation fix-wave (F-P7-01, F-P7-02) | COMPLETE+VERIFIED | Comment-only fix; 61/61 passed, 0 skipped; FEAT_SHA ee89580 |
| Adversarial convergence pass-8 | NOT CLEAN (remediation not clean-pass) | Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-021/D-022/D-023 supplied as ground truth; 2 findings ESCALATED (F-P8-01, F-P8-02); REMEDIATED at FEAT_SHA ad75a7f (D-025 comment-only); convergence streak 0/3. |
| Pass-8 remediation fix-wave (F-P8-01, F-P8-02) | COMPLETE+VERIFIED | Comment-only fix; 61/61 passed, 0 skipped; FEAT_SHA ad75a7f (diff ee89580..ad75a7f: 10 insertions/7 deletions, all comment lines) |
| Adversarial convergence pass-9 | CLEAN | Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-023/D-024/D-025 supplied as ground truth; 0 findings in perimeter; grep-predicate coverage: "Red Gate"=1 sanctioned-provenance, "dangling"=1 corrected, "cycle"=14 body-consistent, "should fail"=0, etc. Convergence streak 1/3 substantiated. PR packaging remains BLOCKED (not 3/3).
| Adversarial convergence pass-10 | CLEAN | Fresh-context different-model static adversary (scoped-to-fix), policies.yaml v1.3 rubric; D-016/D-019/D-023/D-024/D-025 supplied as ground truth; 0 findings in perimeter; orchestrator-executed reached-counts substantiate active inspection (not a ghost pass); convergence streak 2/3 substantiated; PR packaging BLOCKED (not 3/3).
| Adversarial convergence pass-11 | NOT CLEAN | Fresh-context different-model static adversary (scoped-to-fix, Read/Grep/Glob only), policies.yaml v1.3 rubric; D-016/D-019/D-023/D-025 supplied as ground truth; 1 MEDIUM finding (F-P11-01) ESCALATED-PENDING-OPERATOR; predicate reached-counts orchestrator-reconciled (all match ground truth); convergence streak reset 2->0 provisional; NO fix (escalate-before-fix); clean-stop.

## Convergence Status

- Consecutive clean passes: 1 of 3 (Pass 12 CLEAN @45c30f6 D-028). S-1.01 convergence proceeding toward 3/3 (2 more clean passes required). PR packaging remains BLOCKED until 3/3 substantiated.
- Pass 1: ADJUDICATED-REMEDIATED (D-008 F-02/F-03 accept+defer to BC-2.08.004/SS-05; D-009 F-04 guard; D-010 F-01 oracle). F-01 & F-04 RESOLVED + independently gate-verified; F-02/F-03 ADJUDICATED-DEFERRED.
- Pass 2: NOT CLEAN. Fresh-context different-model static adversary (Read/Grep/Glob only), policies.yaml rubric injected, F-02/F-03 supplied as adjudicated-deferred ground truth and correctly not re-litigated. F-01 ADEQUATE (genuine falsifiable independent-set oracle; honest vacuity + F-02/F-03 deferral comments). F-04 mechanism ADEQUATE vs literal D-009/POL-11 checklist but 2 MEDIUM honesty/completeness gaps in the fix; plus 2 MEDIUM latent implementation gaps. Findings are static adversary hypotheses pending operator adjudication.
- Remediation fix-wave at HEAD 4820ead: CI-equiv gate GREEN (build/fmt/clippy -Dwarnings/nextest all exit 0); all 4 Pass-2 MEDIUM findings dispositioned: F-04-a RESOLVED+VERIFIED (D-011), F-04-b ACCEPT+RESIDUAL-DOCUMENTED (D-012), F-VP017 FIXED (D-013), F-SCAN-DOT-ROOT PREMISE-DISPROVEN+COMMENT-FIXED (D-016 opt A).
- Pass 3: NOT CLEAN (fix-wave - remediation, not clean-pass). Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-007/D-016 supplied as ground truth; 4 findings adjudicated at C-#7 (F-P3-01 to F-P3-04); 2 REMEDIATED+VERIFIED (F-P3-01, F-P3-02), 2 did NOT survive re-derivation as material; convergence streak 0/3.
- Pass-3 fix wave at HEAD 46101ae: 61/61 passed, 0 skipped; gate GREEN.
- Pass 4: NOT CLEAN (fix-wave - remediation, not clean-pass). Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-016/D-017 supplied as ground truth; 1 MEDIUM finding (F-P4-01) ESCALATED and REMEDIATED at f468bd5; 2 LOW residuals non-blocking (comment "independent" over-claim; scanner.rs:31 terse-comment clarity); convergence streak 0/3.
- Pass-4 fix wave at HEAD f468bd5: 61/61 passed, 0 skipped; gate GREEN.
- Pass 5: NOT CLEAN (fix-wave - remediation, not clean-pass). Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-018/D-019 supplied as ground truth; 1 finding (F-P5-01) ESCALATED and REMEDIATED at 9d1a6bb (comment-only fix); convergence streak 0/3.
- Pass-5 fix wave at HEAD 9d1a6bb: 61/61 passed, 0 skipped; gate GREEN; comment-only fix for "independent" over-claim.
- Pass 6: CLEAN (first substantiated clean pass). Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-019 supplied as ground truth; 0 findings (F-P5-01 comment resolved). Convergence 1 of 3 substantiated streak achieved at 9d1a6bb; 2 more clean passes required.
- Pass 7: NOT CLEAN. Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-019/D-021 supplied as ground truth; 2 findings ESCALATED (F-P7-01, F-P7-02) ESCALATED-PENDING-OPERATOR; convergence streak reset 1→0. REMEDIATED at FEAT_SHA ee89580 (D-023 comment-only); convergence streak 0/3.
- Pass 8: NOT CLEAN (remediation not clean-pass). Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-021/D-022/D-023 supplied as ground truth; 2 findings ESCALATED (F-P8-01, F-P8-02); REMEDIATED at FEAT_SHA ad75a7f (D-025 comment-only); convergence streak 0/3.
- Pass 9: CLEAN (first substantiated clean pass of Pass 8 remediation). Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric; D-023/D-024/D-025 supplied as ground truth; 0 findings in perimeter; grep-predicate coverage: "Red Gate"=1 sanctioned-provenance, "dangling"=1 corrected, "cycle"=14 body-consistent, "should fail"=0, etc. Convergence streak 1/3 substantiated. PR packaging remains BLOCKED (not 3/3).
- Pass 10: CLEAN (second substantiated clean pass). Fresh-context different-model static adversary (scoped-to-fix), policies.yaml v1.3 rubric; D-016/D-019/D-023/D-024/D-025 supplied as ground truth; 0 findings in perimeter. Orchestrator independently re-ran the adversary's reached-count predicates by execution and CONFIRMED the verdict: all load-bearing single-hit predicates verified; reconciled 2 non-material count discrepancies (adversary 'Regression guard'=6 missed one honest label at L1069 -> actual 7; adversary 'FORBIDDEN_PATTERNS'=10 vs 9 case-sensitive/13 case-insensitive). Both adversary NOT-COVERED gaps closed: HEAD-SHA via orchestrator git rev-parse (=ad75a7f), 61/61 by construction (ee89580..ad75a7f diff comment-only, 10 ins/7 del all comment lines; ee89580 already 61/61-verified). Convergence streak 2/3 substantiated. PR packaging remains BLOCKED (not 3/3).

- Pass 11: NOT CLEAN. Fresh-context different-model static adversary (scoped-to-fix, Read/Grep/Glob only), policies.yaml v1.3 rubric; D-016/D-019/D-023/D-025 supplied as ground truth. 1 MEDIUM finding F-P11-01: scanner_discovery_tests.rs L616/L655 assert "the cycle is detected" but scanner.rs:26 uses follow_links(false), so the walker never enters the cycle and performs NO detection; termination is by not-following. Orchestrator independently execution-verified (scanner.rs:26 .follow_links(false); exactly 2 "detect" hits L616/L655; internal inconsistency L149-152 "vacuous under follow_links(false)"; accurate siblings L645/L1166/L1229) and reconciled all Pass-11 predicate reached-counts against orchestrator ground truth (all match). ESCALATED-PENDING-OPERATOR; NO fix this session (escalate-before-fix). Convergence streak reset 2->0 provisional.
- Pass 12: CLEAN (first substantiated clean pass of the D-028 fix; convergence streak 1 of 3). Fresh-context different-model static adversary (scoped-to-fix, Read/Grep/Glob only), policies.yaml v1.3 rubric; perimeter pre-enumerated at dispatch (E1-E5); baseline feature HEAD 45c30f6 (D-028 fix commit). VERDICT: CLEAN (0 findings). Convergence streak 1 of 3 substantiated. PR packaging remains BLOCKED until 3/3.

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
| F-01 MEDIUM [content-defect] | test-soundness | ADJUDICATED + RESOLVED + VERIFIED | test_BC_2_01_001_no_duplicate_in_scan_set tautological replaced with GENUINE in-scope assertion (D-010) + BC-2.01.006 vacuity note. Falsifiability PROVEN (phantom file → assert FAIL left:2 right:3). Commit 3b705eb (fmt 2859e03). |
| F-02 MEDIUM [content-defect] | partial AC coverage | ADJUDICATED-DEFERRED | AC-006 (story:103-107) and AC-010 (story:124-129) each assert two postconditions; the "anchor table still built via Pass 1.5" half is structurally undischargeable in S-1.01 (no AnchorIndex/run_scan/Pass 1.5 code). Tests verify only the "not in scan set" half. Operator ruling (D-008): DEFERRED to BC-2.08.004 / SS-05 story (NOT blocking for Pass 2). Test comments corrected in 3b705eb to state the deferral honestly. |
| F-03 MEDIUM [content-defect] | semantic anchoring | ADJUDICATED-DEFERRED | POLICY 4 FAIL: VP-016 source-of-truth H1 "Ignored Files Have Anchor Tables", 5 fixtures require run_scan/Pass 1.5/AnchorIndex. Story:71-72 mischaracterizes VP-016 as ".gitignore exclusion — files never in scan set". test_VP_016_* verifies only the exclusion premise, not anchor-target-resolution. VP-016's module (anchor_table) != story target_module (scanner). Operator ruling (D-008): DEFERRED to BC-2.08.004 / SS-05 story (NOT blocking for Pass 2). Test comments corrected in 3b705eb to state the deferral honestly (removed VP-016 over-claim). |
| F-04 MEDIUM [process-gap] | pure-core enforcement absent | ADJUDICATED + RESOLVED + VERIFIED | Mechanical pure-core I/O guard at crates/mdlinkcheck-core/tests/pure_core_guard.rs (D-009). POL-11 positive-coverage (reached-count=2, types.rs+lib.rs, asserted nonzero + closed enumeration). File-scan falsifiability PROVEN (injected use std::fs → exit 101; reverted → exit 0). Commit 710d09b (amended fmt-clean). |
| F-04-a MEDIUM [content-defect] | pure-core guard | ADJUDICATED ACCEPT+FIX — VERIFIED (D-011) | pure_core_guard.rs:46 enumeration now recursive (collect_rs_files_recursively); docstring corrected to "Recursively scans all .rs files"; closed-enumeration (types.rs) + reached-count (N>0) preserved. |
| F-04-b MEDIUM [content-defect] | pure-core guard | ADJUDICATED ACCEPT+FIX — RESIDUAL DOCUMENTED (D-012/D-016) | Per-pattern differential probe now asserts each of the 8 FORBIDDEN_PATTERNS individually via synthetic positives. Residual: probe and matcher share the FORBIDDEN_PATTERNS constant, so it proves each pin is live but not that the pattern set is canonically correct (no independent positives). Residual = operator strengthening question, not a gate failure. |
| F-VP017 MEDIUM [content-defect] | property-test | ADJUDICATED ACCEPT+FIX — VERIFIED (D-013/D-016) | Proptest test_VP_017_proptest_scan_terminates_for_bounded_tree_with_symlink_cycle written (bounded depth + symlink cycle); 4 clippy needless-borrow errors removed at 4820ead; gate GREEN. |
| F-SCAN-DOT-ROOT MEDIUM [content-defect, latent] | scanner filter | ADJUDICATED ACCEPT+FIX — PREMISE DISPROVEN+COMMENT-FIXED (D-016 opt A) | (a) The `ignore` crate does NOT apply filter_entry to the ROOT entry, so a dot-prefixed root IS scanned; D-014's premise ("dot-root -> scan silently empty") is FALSE; no production scanner bug exists; no scanner code was changed this wave. (b) test-writer was told to write a FAILING Red Gate test OR stop-and-escalate; instead it wrote a PASSING test (test_F_SCAN_DOT_ROOT_dot_prefixed_root_dir_is_scanned) carrying a FALSE explanatory comment (claims filter_entry "skip[s] the entry from output while still descending into it"). Operator ruling (D-016): OPTION A — keep test + correct false comment + NO scanner change. No scanner change; comment corrected. |
| F-P3-01 MEDIUM [content-defect] | pure-core guard | RESOLVED+VERIFIED | D-012: FORBIDDEN_PATTERNS entry "rand::rng" dropped (subsumed under .any(contains)). Line-97 claim corrected to "7 pins individually live". 7 pins mutually non-subsuming (exit 0); checker proven non-vacuous (flags old 8-pin set, exit 1). |
| F-P3-02 MEDIUM [content-defect] | oracle robustness | RESOLVED+VERIFIED | POL-11: prop_assert!(!results.is_empty()) after VP-017 scan; all 8 fixture .ok() to .expect(...). Broken-fixture control RED exit 100; clean fixture GREEN. |
| F-P4-01 MEDIUM [content-defect] | test-writer | RESOLVED+VERIFIED | pure_core_guard.rs passed GREEN on an emptied FORBIDDEN_PATTERNS (probe loop + .any() both iterate the const; only runtime N>0 assertion counted FILES not PINS) — the "green-on-emptied-input" vacuity class. Fix: Add fail-closed `assert!(!FORBIDDEN_PATTERNS.is_empty())` + runtime pin-probe positive-coverage count (POL-11 form). D-018 governs; remediated at FEAT_SHA f468bd5; red-on-empty exit 101 / real-set exit 0 / 7 probed 2 validated / 61-61 CI gate. |
| F-P5-01 MEDIUM [content-defect] | test-writer | RESOLVED+VERIFIED | pure_core_guard.rs comment "independent" over-claim: claim "this proves the canonical set is complete" overstates; actual test uses synthetic positives derived from same FORBIDDEN_PATTERNS constant, so it proves mechanism liveness but NOT canonical completeness (D-012 residual). D-019 governs; remediated at FEAT_SHA 9d1a6bb; comment-only fix (no code change); 61-61 CI gate. NULL disposition on scanner.rs Target-2 (comment already accurate; no change needed). |
| F-P7-01 MEDIUM [content-defect] | test-writer | RESOLVED+VERIFIED | Stale "MUST FAIL/BUG" Red-gate comments in scanner_discovery_tests.rs H1 L1010/L1029 + H2 L1050/L1070 (comment-drift; sibling F-SCAN-DOT-ROOT corrected under D-016, H1/H2 un-propagated). D-023 governs; remediated at FEAT_SHA ee89580; comment-only fix; 61-61 CI gate. |
| F-P7-02 LOW [test-comment-accuracy] | test-writer | RESOLVED+VERIFIED | EC-008 comment claims cycle "a->b->a" but only single dangling dir-symlink a->b exists (no reciprocal back-edge). Documentation-only; termination assertion still meaningful. D-023 governs; remediated at FEAT_SHA ee89580; comment-only fix; 61-61 CI gate. |
| F-P8-01 MEDIUM [test-documentation-accuracy] | test-writer | RESOLVED+VERIFIED | EC-008 comments in scanner_discovery_tests.rs had DIRECTION-WRONG framing: claimed "dangling symlink a->b (b nonexistent)" when create_dir_symlink(&dir_a,&dir_b) actually creates a link AT dir_b pointing TO dir_a (b->a), target a exists, NOT dangling. D-025 governs (new binding evidence rule: fix author MUST verify claimed behavior against executed/documented semantics and STATE that verification in its report). Remediated at FEAT_SHA ad75a7f (diff ee89580..ad75a7f: 10 insertions/7 deletions, all comment lines); 61-61 CI gate. |
| F-P8-02 LOW [test-comment-accuracy] | test-writer | RESOLVED+VERIFIED | F-SCAN-DOT-ROOT guard block header + inline still framed a PASSING test as a "Red Gate test". D-025 governs; remediated at FEAT_SHA ad75a7f (comment-only, relabelled "Regression guard" with provenance note); 61-61 CI gate.
| F-P11-01 MEDIUM [test-comment-accuracy] | test-writer | ESCALATED-PENDING-OPERATOR | scanner_discovery_tests.rs L616/L655 (test_BC_2_01_001_scan_terminates_with_genuine_symlink_cycle): comments assert "the cycle is detected and avoided" / "should detect and handle the cycle", but scanner.rs:26 uses .follow_links(false) — the ignore walker never descends into the symlinked dirs, never enters the cycle, performs NO detection; termination is by not-following. D-025 binding-evidence-rule + POL-4. Orchestrator execution-verified (follow_links(false) at scanner.rs:26; 2 "detect" hits; L149-152 corroboration; siblings L645/L1166/L1229 accurate). NO fix (escalate-before-fix). |

## Decisions Log

| ID | Date | Decision |
|----|------|----------|
| D-001 | 2026-08-18 | Phase 3 initialized from the ratified spec package at develop f81f412. Engagement scope: waves 1→4 per sprint-state.yaml; wave gates HUMAN-ratified; merges/verdicts/PR creation HUMAN-executed. |
| D-002 | 2026-08-18 | Operator ruling: pin clap="=4.6.5", unicode-normalization="=0.1.24", proptest="~1.6" (resolved 1.6.0) to match ratified dependency-graph.md verified-version table. |
| D-003 | 2026-08-18 | Orchestrator decision (process): removed out-of-scope `ureq="3.3.0"` from S-1.01 binary manifest. Pruned entire rustls/ring/webpki TLS subtree (282 lock lines). |
| D-004 | 2026-08-18 | Orchestrator decision (process): accepted test-writer's addition of crates/mdlinkcheck/src/lib.rs (`pub mod scanner;`) + scanner.rs `use std::path::PathBuf;` as minimal enabler for integration tests. |
| D-005 | 2026-08-19 | Operator ruling: fix tests to spec-correct oracles for the two gitignore over-exclusion tests (VP-016 and mixed-scenario); test-file-only change. Commit 3e1f253. |
| D-006 | 2026-08-19 | Process: expose CliArgs via the lib crate (crates/mdlinkcheck/src/cli.rs, pub) so AC-008/F5 can assert the CLI surface has no --hidden. |
| D-007 | 2026-08-19 | Operator-ruled durable: PC3 file-symlink following DEFERRED to BC-2.01.006; S-1.01 fix = correct the false comment + record tech-debt deferral entry. |
| D-008 | 2026-08-19 | Operator ruling — F-02/F-03 ACCEPT + DEFER, no spec edit: VP-016 (source_bc BC-2.08.004, module anchor_table) and "anchor table still built via Pass 1.5" halves of AC-006 and AC-010 are formally DEFERRED to BC-2.08.004 / SS-05 story (NOT blocking for Pass 2). |
| D-009 | 2026-08-19 | Operator ruling — F-04: Add mechanical pure-core I/O guard NOW, test/CI-only, with POL-11 positive-coverage assertion. File-scan falsifiability PROVEN (injected use std::fs → exit 101; reverted → exit 0). Commit 710d09b. |
| D-010 | 2026-08-19 | Operator ruling — F-01: Test-only honest fix — replace tautological dedup oracle with GENUINE in-scope assertion (D-010) + BC-2.01.006 vacuity note. No spec edit. Commit 3b705eb. |
| D-011 | 2026-08-19 | Operator ruling — F-04-a ACCEPT + FIX (test/CI-only): pure_core_guard.rs scans core/src NON-recursively while docstring claims "all .rs files". Fix = make the .rs enumeration recursive over core/src AND update docstring; keep closed-enumeration (types.rs present) + reached-count. No spec edit. VERIFIED. |
| D-012 | 2026-08-19 | Operator ruling — F-04-b ACCEPT + FIX (test/CI-only): differential probe exercised only 1/8 forbidden patterns under an .any() matcher (liveness not pin-completeness). Fix = assert each of the 8 FORBIDDEN_PATTERNS individually via synthetic positives (per-pattern differential probe). RESIDUAL DOCUMENTED. |
| D-013 | 2026-08-19 | Operator ruling — F-VP017 ACCEPT + FIX (test-only): frozen story Task10 + frozen BC-2.01.001 Proof Method both mandate proptest (bounded depth + symlink cycle); delivered as example cases (proptest declared, never invoked). Fix = write the real proptest so S-1.01 satisfies its own frozen VP. FUNCTIONAL — BUT introduces 4 clippy needless-borrow errors. REMEDIATED at 4820ead (clippy errors removed). |
| D-014 | 2026-08-19 | Operator ruling — F-SCAN-DOT-ROOT ACCEPT + FIX — PREMISE DISPROVEN (D-014): scanner.rs filter_entry rejects any '.'-prefixed entry incl. ROOT; root="." → entire scan silently empty. Fix = exempt root entry (depth 0) from dot-dir pruning, failing-test-first. PREMISE DISPROVEN: ignore crate does NOT apply filter_entry to ROOT entry. FALSE POSITIVE + test-writer deviation (passing test with false comment). RE-ADJUDICATION NEEDED: option A keep test + correct false comment + NO scanner change; option B add defensive depth-0 root exemption anyway. |
| D-015 | 2026-08-19 | SEC-01 RESOLVED by human ruling — AUTHORIZE PUSHES of factory state to public origin drbothen/mdlinkcheck-spark. EXCEPTION-01 and EXCEPTION-02 CLOSED as human-ruled. Resume normal commit+push with ls-remote proof at every checkpoint. |
| D-016 | 2026-08-19 | Operator ruling on F-SCAN-DOT-ROOT re-adjudication (D-014 premise DISPROVEN) → OPTION A: NO scanner change. The "dot-root -> empty scan" premise is disproven and confirmed by orchestrator execution (test_F_SCAN_DOT_ROOT passes: dot-prefixed root IS scanned because the `ignore` crate does not apply filter_entry to the root entry). No production scanner bug exists. Fix = correct the false test comment only (mechanism restated accurately); keep the passing test. Also operator ruling on F-04-b residual → OPTION A: accept per-pattern synthetic-positive probe as-is; residual (probe/matcher share FORBIDDEN_PATTERNS constant) documented as a strengthening question, not a gate failure (D-012 stands). |
| D-017 | 2026-08-19 | Operator ruling at gate C-#7 — Pass-3 adjudication: F-P3-01 ACCEPT+FIX drop-subsumed-pin; F-P3-02 ACCEPT+FIX non-empty-assert+loud-fixtures; both test-file-only, no spec edit. 2 prior transcript-only Pass-3 items did NOT survive re-derivation as material (one was "reaffirm D-007" — already durably satisfied in blocking-issues-resolved.md + cycle-manifest TD-002 + scanner.rs:25 corrected comment; the F-SCAN-DOT-ROOT comment is D-016-adequate). |
| D-018 | 2026-08-19 | Operator ruling — F-P4-01 ACCEPT+FIX (test-only): Add fail-closed `assert!(!FORBIDDEN_PATTERNS.is_empty())` + runtime pin-probe positive-coverage count (POL-11 form). Pure-core guard passed GREEN on emptied FORBIDDEN_PATTERNS (vacuity class "green-on-emptied-input"). Fix: assert non-empty + runtime positive-coverage count. Test-only; no spec edit. COMMITTED at FEAT_SHA f468bd5; red-on-empty exit 101 / real-set exit 0 / 7 probed 2 validated / 61-61 CI gate. |
| D-019 | 2026-08-20 | Operator ruling — F-P5-01 ACCEPT+FIX comment-only (test-only): Comment "independent" over-claim in pure_core_guard.rs — claim that synthetic positive test proves canonical set completeness is false; test derives positives from same FORBIDDEN_PATTERNS constant, proving only mechanism liveness, not canonical completeness (D-012 residual). Fix: Correct comment to state accurate claim. NO code change. NULL disposition on scanner.rs Target-2 (comment already accurate; no change). Commit 9d1a6bb. |
| D-021 | 2026-08-20 | S-1.01 convergence checkpoint — 3/3 clean passes achieved at 9d1a6bb; CI-wiring verified (POL-11 in CI); 61/61 tests green; demo evidence registered (13 ACs x 3 formats); NEXT: pr-manager packaging for HUMAN execution. |
| D-020 | 2026-08-20 | Operator ruling this session: Convergence gate honesty correction - transcript-only adversarial passes are uncertified and do not count; S-1.01 substantiated clean-pass streak reset from over-claimed 3/3 to 1/3 (final clean pass at 9d1a6bb); remaining 2 clean passes run one-per-fresh-session, each enumerated + committed before the next; PR packaging deferred until 3/3 durably enumerated. Also note the process-gap: pr-manager template auto-filled fabricated coverage/mutation/holdout/security figures and an invented adversarial findings table (no Pass 7 exists); flag for lessons codification. |
| D-022 | 2026-08-20 | Adversarial Pass 7 (fresh-context different-model static, scoped-to-fix) at feature 9d1a6bb: NOT CLEAN. F-P7-01 MEDIUM (stale 'MUST FAIL/BUG' Red-gate comments in scanner_discovery_tests.rs H1 L1010/L1029 + H2 L1050/L1070 assert the impl is buggy while both plain #[test]s pass at the 61/61-green 9d1a6bb gate — same comment-drift class as D-016/D-019, un-propagated to H1/H2). F-P7-02 LOW (EC-008 comment claims a cycle it does not construct). Both ESCALATED-PENDING-OPERATOR; NO fix this session (escalate-before-fix + Pass-7-only clean-stop). Convergence streak reset 1->0 of 3 (provisional; operator may adjudicate F-P7-01 immaterial -> 2/3). Orchestrator independently inspection-verified both findings. Checkpoint committed LOCAL-ONLY; push deferred to operator per session rider. |
| D-023 | 2026-08-20 | Operator ruling this session: F-P7-01/F-P7-02 ACCEPT+FIX comment-only. Remediation committed at ee89580: stale Red-gate comments corrected (H1/H2) + EC-008 false-cycle comment corrected. 61/61 CI gate GREEN. D-022 verdict (NOT CLEAN) retroactively replaced with REMEDIATED+VERIFIED. Convergence streak 0/3. NEXT: Adversarial Pass 8 (fresh context). |
| D-024 | 2026-08-20 | Operator ruling this session: Adversarial Pass 8 at feature ee89580 NOT CLEAN. F-P8-01 MEDIUM (EC-008 direction-wrong framing: claimed "dangling symlink a->b (b nonexistent)" but create_dir_symlink(&dir_a,&dir_b) creates link AT dir_b pointing TO dir_a (b->a), target a exists, NOT dangling). F-P8-02 LOW (F-SCAN-DOT-ROOT guard block header + inline still framed a PASSING test as a "Red Gate test"). Both ESCALATED-PENDING-OPERATOR; convergence streak reset 0/3. |
| D-025 | 2026-08-20 | Operator ruling this session: F-P8-01/F-P8-02 ACCEPT+FIX comment-only. REMEDIATION COMMITTED AT ad75a7f: (1) EC-008 comment corrected with direction-accurate wording ("directory symlink b -> a; target a exists; not followed; scan must terminate") with verification statement per NEW BINDING EVIDENCE RULE: fix author MUST verify claimed behavior against executed/documented semantics (here: std::os::unix::fs::symlink(src,dst) creates link AT dst pointing TO src) and STATE that verification in its report. (2) F-SCAN-DOT-ROOT relabelled as "Regression guard" with honest provenance note ("originally authored as a Red Gate probe; premise disproven under D-016; retained as a regression guard"). Code message-strings (the .expect("create symlink a->b") at ~L861) remain OUT OF SCOPE. 61/61 CI gate GREEN. Convergence streak 0->1 of 3 substantiated. NEXT: Adversarial Pass 9 (fresh context, scoped-to-fix). PR packaging remains BLOCKED until 3/3 substantiated. |
| D-026 | 2026-08-20 | Adversarial Pass 10 (fresh-context different-model static, scoped-to-fix) at feature ad75a7f: CLEAN, 0 findings in perimeter. Orchestrator execution-verified the adversary's reached-count predicates (confirmed all load-bearing single-hit predicates; reconciled 2 non-material count discrepancies; closed both adversary NOT-COVERED gaps — HEAD-SHA via git rev-parse=ad75a7f, 61/61 by comment-only-diff construction). Convergence streak 1/3 → 2/3 substantiated. NO fix wave (clean pass). PR packaging remains BLOCKED until 3/3. NEXT: one more fresh-context clean pass. |

| D-027 | 2026-08-20 | Adversarial Pass 11 (fresh-context different-model static, scoped-to-fix) at feature ad75a7f: NOT CLEAN. F-P11-01 MEDIUM (test-comment-accuracy): scanner_discovery_tests.rs L616/L655 assert the symlink cycle is "detected", contradicting scanner.rs:26 .follow_links(false) (walker never enters the cycle; termination is by not-following, NOT by detection). Same comment-drift class as F-P7/F-P8, under D-025 binding evidence rule + POL-4. ESCALATED-PENDING-OPERATOR; NO fix this session (escalate-before-fix + Pass-11-only clean-stop). Orchestrator independently execution-verified the finding (follow_links(false) confirmed; only 2 "detect" hits at L616/L655; internal inconsistency at L149-152; accurate sibling phrasing at L645/L1166/L1229) and reconciled all Pass-11 predicate reached-counts against orchestrator ground truth (all match). Convergence streak reset 2->0 of 3 (PROVISIONAL; operator may adjudicate F-P11-01 immaterial -> restore 2/3, per D-022 precedent). Checkpoint committed LOCAL-ONLY via github-ops direct; push deferred to operator per session rider. NO second pass this session. |
| D-028 | 2026-08-20 | Operator decision this session: F-P11-01 adjudicated OPTION A (ACCEPT+FIX, comment-only): correct scanner_discovery_tests.rs L616 & L655 to describe follow_links(false) not-following termination. Option B (immaterial -> restore streak 2/3) REJECTED: comment over-claims ruled material at F-P5-01/F-P7-01/F-P8-01; a fourth restoration for the same class would relax the bar (C-#8 forbids). Convergence streak CONFIRMED reset 0/3; 3 fresh clean passes required from the fix. Governed by D-025 binding-evidence rule. |

## Skip Log

| ID | Date | Skipped | Reason |
|----|------|---------|--------|

## Blocking Issues

| ID | Issue | Severity | Blocking Phase | Owner | Resolution |
|----|-------|----------|---------------|-------|------------|
| F-02 | AC-006 and AC-010 partial coverage — "anchor table still built via Pass 1.5" half structurally undischargeable in S-1.01. | MEDIUM | phase-3 | test-writer | ADJUDICATED-DEFERRED by operator (D-008); DEFERRED to BC-2.08.004 / SS-05 story (NOT blocking for Pass 2). |
| F-03 | VP-016 semantic anchoring POLICY 4 FAIL — VP-016 source-of-truth H1 "Ignored Files Have Anchor Tables". | MEDIUM | phase-3 | spec-steward | ADJUDICATED-DEFERRED by operator (D-008); DEFERRED to BC-2.08.004 / SS-05 story (NOT blocking for Pass 2). |

## Drift Items

- [clippy-gap] F-VP017 proptest introduced 4 clippy needless-borrow errors (scanner_discovery_tests.rs lines 1242, 1243, 1248, 1249). MECHANICAL FIX COMPLETE at 4820ead; gate GREEN.
- [feature-branch] Local feature HEAD is 45c30f6 (D-028 fix wave: comment-only L616/L655 fix applied + verified 61/61). Origin is 53ffff4. Pass-8/D-024 remediation at ee89580, Pass-9/D-025 fix-wave at ad75a7f, Pass-10/D-026 clean pass, Pass-11/D-027 NOT CLEAN, Pass-12/D-028 CLEAN.

## Session Resume Checkpoint

S-1.01 convergence: Adversarial Pass 12 CLEAN @45c30f6 (D-028). Fresh-context different-model static adversary (scoped-to-fix, Read/Grep/Glob only), policies.yaml v1.3 rubric; perimeter pre-enumerated at dispatch (E1-E5); baseline feature HEAD 45c30f6 (D-028 fix commit). VERDICT: CLEAN (0 findings). Convergence streak 1 of 3 substantiated (first clean pass of the D-028 fix). PR packaging remains BLOCKED until 3/3 substantiated. Feature tree updated at 45c30f6 (D-028 comment-only fix L616/L655 applied + verified 61/61). This checkpoint committed LOCAL-ONLY on factory-artifacts via github-ops direct; OPERATOR must push and return ls-remote proof. NEXT SESSION: run the next fresh-context scoped-to-fix adversarial pass (Pass 13) at 45c30f6 toward streak 2/3. Convergence achieved only at 3 consecutive clean passes.

## Adversarial Pass 8

- Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric
- D-021/D-022/D-023 supplied as ground truth
- **Findings:** 2 MEDIUM (F-P8-01), 1 LOW (F-P8-02)
- **Operator Ruling:** D-024 — Both findings ESCALATED-PENDING-OPERATOR; convergence streak 0/3
- **Remediation:** Test-only comment-only fix pair committed at FEAT_SHA ad75a7f (diff ee89580..ad75a7f: 10 insertions/7 deletions, all comment lines)
- **Gate Verification:** 61/61 passed, 0 skipped, gate GREEN
- **Verdict:** REMEDIATED+VERIFIED, NOT CLEAN (remediation, not clean-pass)
- **Convergence streak:** 0 of 3
- **New binding evidence rule:** Fix author MUST verify claimed behavior against executed/documented semantics and STATE that verification in its report (F-P8-01)

### Pass 8 Adversarial Findings

| ID | Severity | Category | Issue | Notes |
|----|----------|----------|-------|-------|
| F-P8-01 | MEDIUM | test-documentation-accuracy | EC-008 comment claims "dangling symlink a->b (b nonexistent)" but create_dir_symlink(&dir_a,&dir_b) creates link AT dir_b pointing TO dir_a (b->a), target a exists, NOT dangling | Comment-only. D-025 governs (new binding evidence rule: fix author MUST verify claimed behavior against executed/documented semantics and STATE that verification in its report). Remediated at FEAT_SHA ad75a7f with direction-correct wording: "directory symlink b -> a; target a exists; not followed; scan must terminate" + verification statement. |
| F-P8-02 | LOW | test-comment-accuracy | F-SCAN-DOT-ROOT guard block header + inline still framed a PASSING test as a "Red Gate test" | Comment-only. D-025 governs; remediated at FEAT_SHA ad75a7f with relabelled "Regression guard" and honest provenance note: "originally authored as a Red Gate probe; premise disproven under D-016; retained as a regression guard" |

## Adversarial Pass 9

- Fresh-context different-model static adversary (scoped-to-fix), policies.yaml rubric
- D-023/D-024/D-025 supplied as ground truth
- **Findings:** 0 (all previous findings REMEDIATED+VERIFIED)
- **Verdict:** CLEAN (first substantiated clean pass of Pass 8 remediation)
- **Convergence streak:** 1 of 3 (substantiated)
- **Grep-predicate coverage evidence:** "Red Gate"=1 sanctioned-provenance, "dangling"=1 corrected, "cycle"=14 body-consistent, "should fail"=0
- **Governing decisions:** D-023 (F-P7-01/F-P7-02), D-024 (F-P8-01/F-P8-02), D-025 (F-P8-01/F-P8-02 comment-only fix + new binding evidence rule)

## Adversarial Pass 10

- Fresh-context different-model static adversary (scoped-to-fix), policies.yaml v1.3 rubric
- D-016/D-019/D-023/D-024/D-025 supplied as ground truth; D-025 out-of-scope `.expect("create symlink a->b")` string honored
- **Findings:** 0 (all four targets A1 EC-008 comments / A2 F-SCAN-DOT-ROOT guard block / A3 H1/H2 regression comments incl. L1034 / B1 pure_core_guard comment set CLEAN)
- **Verdict:** CLEAN (second substantiated clean pass)
- **Convergence streak:** 2 of 3 (substantiated)
- **Reached-count evidence (orchestrator-executed; grep lower bounds):** scanner_discovery_tests.rs — "Red Gate"=1 (L1281 honest historical provenance), "dangling"=1 (L858 negated "not dangling"), "MUST FAIL"=0, "should fail"=0, "Regression guard"=7 (all honest PASSING-guard labels), "cycle"=23, "BUG"=3 (L310 prod gitignore-masking / L1034 H1 regression-target message / L1287 "no production scanner bug"). pure_core_guard.rs — "independent"=0, "canonical"=1 (L104 negated completeness disclaimer), "complete"=1 (L104), "liveness"=0, "FORBIDDEN_PATTERNS"=9 case-sensitive / 13 case-insensitive.
- **Orchestrator verification note:** verdict CONFIRMED by independent execution; 2 non-material count discrepancies reconciled; both adversary NOT-COVERED gaps (HEAD-SHA, 61/61) closed as described in Convergence Status Pass 10 bullet.
- **Governing decision:** D-026.

## Adversarial Pass 11

- Fresh-context different-model static adversary (scoped-to-fix, Read/Grep/Glob only), policies.yaml v1.3 rubric
- Baseline feature HEAD ad75a7f; perimeter = comments in scanner_discovery_tests.rs + pure_core_guard.rs
- D-016/D-019/D-023/D-025 supplied as ground truth (not re-litigated)
- **Verdict:** NOT CLEAN (1 MEDIUM finding)
- **Finding:** F-P11-01 MEDIUM (test-comment-accuracy) — scanner_discovery_tests.rs L616 ("The scanner should detect and handle the cycle") and L655 ("the cycle is detected and avoided") mischaracterize the termination mechanism; scanner.rs:26 uses .follow_links(false), so the ignore walker never descends into the symlinked dirs, never enters the cycle, performs NO cycle detection. Termination is guaranteed by not-following, not by detection. D-025 binding-evidence-rule + POL-4.
- **Orchestrator execution-verification (finding CONFIRMED material):** scanner.rs:26 .follow_links(false) confirmed; exactly 2 "detect" hits in file, both at L616/L655; internal inconsistency confirmed (L149-152 correctly reasons behavior is "VACUOUS under follow_links(false)"); sibling comments L645/L1166/L1229 use accurate mechanism-neutral phrasing.
- **Predicate reached-counts (orchestrator-reconciled; case-insensitive unless noted):** scanner_discovery_tests.rs — "Red Gate"=1 (L1281 honest), "MUST FAIL"=0, "should fail"=0, "dangling"=1 (L858 honest negated), "regression guard"=7 (all honest), "cycle"=23 line-hits/28 occ (2 DEFECT L616/L655, rest honest), "bug"=3 (L310/L1034/L1287 honest). pure_core_guard.rs — "independent"=0, "canonical"=1 (L104 honest disclaimer), "complete"=1 (L104), "liveness"=0, "FORBIDDEN_PATTERNS"=9 (cs). All match orchestrator ground truth.
- **NOT-COVERED closures (orchestrator-side):** HEAD-SHA=ad75a7f (git rev-parse); finding is comment-only — 61/61 test-pass unaffected (no code change proposed).
- **Disposition:** ESCALATED-PENDING-OPERATOR; NO fix (escalate-before-fix); NO second pass. Convergence streak reset 2->0 provisional.
- **Governing decision:** D-027.

## Adversarial Pass 12

- Fresh-context different-model static adversary (Read/Grep/Glob only), scoped-to-fix lens, policies.yaml v1.3 rubric injected; perimeter pre-enumerated at dispatch (E1-E5).
- Baseline feature HEAD 45c30f6 (D-028 fix commit).
- Fix commit: `test(S-1.01): correct cycle-termination comments (L616/L655) per D-028` @ 45c30f666da16fb2193b91522cc36d72b5abb10c — comment-only, 1 file, 2 insertions/2 deletions. L616 now "The symlinks are not followed, so the cycle is never entered and the scan terminates."; L655 now "Should find both files (the symlinks are not followed, so the cycle is never entered)".
- VERDICT: CLEAN (0 findings). Two Bash-dependent obligations (git-diff scope proof; pure_core_guard.rs unchanged proof) were delegated by the static adversary to the orchestrator.
- Orchestrator execution-verification (verdict CONFIRMED): (a) diff-proof — `git show --stat 45c30f6` = 1 file changed, 2 ins/2 del, both comment lines, NO scope creep; (b) control run — `cargo test --workspace --locked` workspace aggregate 61 passed / 0 failed / 0 ignored, exit 0 (note: test-writer report quoted per-binary 34; orchestrator reconciled to workspace aggregate 61/61); (c) reached-count reconciliation (all MATCH orchestrator ground truth, case-insensitive unless noted): scanner_discovery_tests.rs "detect"=0, "Red Gate"(cs)=1 (L1281 honest provenance), "MUST FAIL"/"should fail"=0, "dangling"=1 (L858 honest negated), "regression guard"=7, "cycle"=23 line-hits, "bug"=3; pure_core_guard.rs (crates/mdlinkcheck-core/tests/) NOT touched by 45c30f6 (unchanged). D-025 binding-evidence satisfied (fix author quoted scanner.rs:26 `.follow_links(false)` + doc L24).
- Convergence streak: 1 of 3 (substantiated, first clean pass of the D-028 fix).
- Governing decision: D-028.

## NEXT ACTION

Feature HEAD is 45c30f6 (D-028 fix applied, UNPUSHED, packaged-for-human). Pass 12 CLEAN, streak 1 of 3. Convergence requires 3 consecutive clean passes; 2 more fresh-context scoped-to-fix clean passes remain (future sessions). PR packaging remains BLOCKED until 3/3 substantiated. This checkpoint is LOCAL-ONLY on factory-artifacts until the operator pushes and returns `git ls-remote origin factory-artifacts` proof matching the new local SHA. Worktree-health gate PASSED this session (only deviation: designed UNPUSHED feature branch, adjudicated expected). NEXT SESSION: run the next fresh-context scoped-to-fix adversarial pass (Pass 13) at 45c30f6 toward streak 2/3.
