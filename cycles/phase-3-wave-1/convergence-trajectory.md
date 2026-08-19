---
document_type: convergence-trajectory
level: ops
version: "1.0"
status: complete
producer: state-manager
timestamp: 2026-08-19T11:09:00Z
cycle: "phase-3-wave-1"
input-hash: "[md5]"
traces_to: STATE.md
---

# Convergence Trajectory — phase-3-wave-1

## Finding Progression

| Pass | Date | Total | CRIT | HIGH | MED | LOW | Novelty | Score | Counter | Verdict |
|------|------|-------|------|------|-----|-----|---------|-------|---------|---------|
| 1 | 2026-08-19 | 4 | 0 | 0 | 4 | 0 | HIGH | 0.00 | 0/3 | ADJUDICATED |

## Trajectory Shorthand

`4→...`

## Per-Pass Details

### Pass 1 (2026-08-19)

**Findings:** 4 (0 CRIT, 0 HIGH, 4 MED, 0 LOW)
**Novelty:** HIGH (fresh-context different-model adversary)
**Convergence counter:** 0/3

**Operator Ruling:** ADJUDICATED per D-008/D-009/D-010

#### Summary

Fresh-context adversarial review of S-1.01 implementation via orchestrator-run execution harness. All findings verified by three-part evidence (spec quote + code location + execution proof).

#### Adversarial Review Findings - OPERATOR RULINGS

| ID | Severity | Category | Issue | Operator Ruling | Status |
|----|----------|----------|-------|-----------------|--------|
| F-01 | MEDIUM | content-defect | test-soundness: scanner_discovery_tests.rs:101-130 dedup test (test_BC_2_01_001_no_duplicate_in_scan_set) is tautological — asserts returned_len == HashSet(returned).len(), structurally cannot fail; with follow_links(false) no file is reachable twice, so BC-2.01.001 PC2 multi-path clause not exercised (dead false-arm). (Reopens prior ledger F4.) | D-010: Replace tautological dedup oracle with GENUINE in-scope assertion comparing returned set against independently-known .md files test created; add vacuity note that BC-2.01.001 PC2 multi-path clause is vacuous under follow_links(false). PENDING fix. | PENDING fix |
| F-02 | MEDIUM | content-defect | partial AC coverage: AC-006 (story:103-107) and AC-010 (story:124-129) each assert two postconditions; the "anchor table still built via Pass 1.5" half is structurally undischargeable in S-1.01 (no AnchorIndex/run_scan/Pass 1.5 code). Tests verify only the "not in scan set" half. | D-008: DEFERRED to story implementing run_scan + Pass 1.5 + AnchorIndex (BC-2.08.004 / SS-05 owner). This is a DOCUMENTED DEFERRAL — no spec edit. Future adversarial passes MUST treat F-02 as ADJUDICATED-DEFERRED (not blocking). | ADJUDICATED-DEFERRED |
| F-03 | MEDIUM | content-defect | semantic anchoring — POLICY 4 FAIL: VP-016 source-of-truth H1 "Ignored Files Have Anchor Tables — Cross-File Anchors into Ignored Files Resolve", source_bc BC-2.08.004, module anchor_table, 5 fixtures require run_scan/Pass 1.5/AnchorIndex. Story:71-72 mischaracterizes VP-016 as ".gitignore exclusion — files never in scan set". test_VP_016_* verifies only the exclusion premise, not anchor-target-resolution. VP-016's module (anchor_table) != story target_module (scanner). | D-008: DEFERRED to story implementing run_scan + Pass 1.5 + AnchorIndex (BC-2.08.004 / SS-05 owner). The POLICY 4 semantic-anchoring inaccuracy in the S-1.01 story's VP-016 description is a known frozen-spec issue carried as tech-debt (spec-text fix deferred). | ADJUDICATED-DEFERRED |
| F-04 | MEDIUM | process-gap | pure-core enforcement absent: story Architecture Compliance table:225 claims std::fs/std::net ban in mdlinkcheck-core enforced by "cargo deny rule" + "Kani harnesses fail to compile if I/O imported". cargo-deny structurally cannot ban std modules; zero Kani harnesses exist. Code currently complies (types.rs imports only std::collections::HashMap, std::path::PathBuf, serde) but no mechanical guard exists. | D-009: Add cheap MECHANICAL pure-core I/O guard NOW, test/CI-only, with POL-11 positive-coverage assertion. Correct enforcement wording in test/CI comment. The frozen S-1.01 compliance-table wording ("cargo deny rule; Kani harnesses") fix is DEFERRED as tech-debt (no spec edit this cycle). PENDING fix. | PENDING fix |

#### Non-blocking Observations (LOW)

| ID | Severity | Category | Issue | Status |
|----|----------|----------|-------|--------|
| OBS-1 | LOW | dev-efficiency | proptest unused dev-dep (VP-017 proof_method=integration; implementer correctly wrote integration test). | VERIFIED |
| OBS-2 | LOW | unused-dep | rayon unused prod dep (ADR-005 parallel deferred). | VERIFIED |
| OBS-3 | LOW | acceptable-stub | main.rs is fn main(){todo!()} (acceptable stub for S-1.01 scope). | VERIFIED |
| OBS-4 | LOW | out-of-scope | filter_entry dot-root basename edge is out-of-scope for default-CWD S-1.01. | VERIFIED |

#### Bookkeeping Note

- STORY-INDEX.md and S-1.01 story frontmatter say status: draft while sprint-state.yaml says S-1.01 in_progress. This discrepancy will be reconciled later.

---

### Pass 2 (2026-08-19) - FIX PAIR COMPLETE

**Findings:** 0 (0 CRIT, 0 HIGH, 0 MED, 0 LOW) - F-02/F-03 injected as ADJUDICATED-DEFERRED non-findings
**Novelty:** LOW (same adversary, fix pair verification pass)
**Convergence counter:** 0/3

**Operator Ruling:** S-1.01 F-01+F-04 fix pair COMPLETE + VERIFIED at HEAD 2859e03

#### Summary

The S-01 fix pair (F-01 genuine oracle, F-04 pure-core guard) has been independently verified via full CI gate (build/fmt/clippy -Dwarnings/nextest all exit 0; 59/59 passed). F-02 and F-03 are ADJUDICATED-DEFERRED per D-008 (not blocking for Pass 2).

#### Fix Pair Verification

| Finding | Status | Verification |
|---------|--------|--------------|
| F-01 | RESOLVED + VERIFIED | GENUINE in-scope dedup assertion + BC-2.01.006 vacuity note; falsifiability PROVEN (phantom file → assert FAIL left:2 right:3); commit 3b705eb (+ fmt 2859e03) |
| F-04 | RESOLVED + VERIFIED | Mechanical pure-core I/O guard at crates/mdlinkcheck-core/tests/pure_core_guard.rs; POL-11 positive-coverage (reached-count=2); file-scan falsifiability PROVEN; commit 710d09b |
| F-02 | ADJUDICATED-DEFERRED | DEFERRED to BC-2.08.004 / SS-05; test comments corrected to state deferral honestly in 3b705eb |
| F-03 | ADJUDICATED-DEFERRED | DEFERRED to BC-2.08.004 / SS-05; test comments corrected to state deferral honestly in 3b705eb |

#### Notes
- This is a FIX PAIR VERIFICATION pass, not a full adversarial pass
- F-02/F-03 are injected as ADJUDICATED-DEFERRED non-findings (per D-008) - must NOT be re-raised as blocking
- 3 consecutive clean passes still required to converge S-1.01 (currently 0/3)
- Convergence trajectory: `4→...→0` (Pass 1: 4 findings, Pass 2: 0 findings due to fix pair + deferrals)

---

### Pass 2 (2026-08-19) - SECOND ADVERSARIAL PASS

**Findings:** 4 MEDIUM (F-04-a, F-04-b, F-VP017, F-SCAN-DOT-ROOT), 0 HIGH, 0 CRIT
**Novelty:** MEDIUM (fresh-context different-model static adversary, policies.yaml rubric)
**Convergence counter:** 0/3
**Verdict:** NOT CLEAN

#### Adversarial Review Findings - PASS 2

| ID | Severity | Category | Issue | Notes |
|----|----------|----------|-------|-------|
| F-04-a | MEDIUM | content-defect | pure_core_guard.rs:46 non-recursive fs::read_dir vs docstring "all .rs files" | Future core/src subdir .rs silently unscanned but still green + positive reached-count; non-recursion limit absent from sensitivity block. Fix: recurse OR reword to "top-level" + add caveat. ESCALATED-PENDING-OPERATOR. |
| F-04-b | MEDIUM | content-defect | pure_core_guard.rs:82-88 probe exercises only 1/8 forbidden patterns; .any() short-circuit means corrupted later pattern never caught | Proves mechanism liveness not pin completeness. Fix: assert each pattern individually. ESCALATED-PENDING-OPERATOR. |
| F-VP017 | MEDIUM | content-defect | Story Task10 (S-1.01:208) + BC-2.01.001.md:74 mandate proptest; delivered as 2 hand cases; proptest never invoked | Example-based not property-based. Frozen spec → tech-debt vs Task10/VP-017. ESCALATED-PENDING-OPERATOR. |
| F-SCAN-DOT-ROOT | MEDIUM | content-defect (latent) | scanner.rs:37-46 filter_entry rejects any '.'-prefixed entry incl. ROOT; root="." → entire scan silently empty | Threatens AC-001 once main wired; H1 test:1013 covers dot-ancestors only. ESCALATED-PENDING-OPERATOR. |

#### Operator Verdict Summary

- F-01 ADEQUATE: genuine falsifiable independent-set oracle; honest vacuity + F-02/F-03 deferral comments
- F-04 mechanism ADEQUATE vs literal D-009/POL-11 checklist but 2 MEDIUM honesty/completeness gaps in the fix
- 2 MEDIUM latent implementation gaps (F-VP017, F-SCAN-DOT-ROOT) pending operator adjudication

#### LOW Advisory Notes

| ID | Severity | Category | Issue |
|----|----------|----------|-------|
| ADV-1 | LOW | implementation-gate | guard enumeration pins types.rs not lib.rs (minor coverage gap) |
| ADV-2 | LOW | edge-case | ".md"-named file extension edge (pending verification) |
| ADV-3 | LOW | comment-nit | symlink-cycle comment nit (non-blocking) |
| ADV-4 | LOW | not-enumerated | SystemTime::now/getrandom not enumerated in POL-11 (not blocking) |

#### Policy Review Summary

- POL-1 (non-empty scan): PASS
- POL-4 (enumerated forbidden patterns): PASS
- AC-008 D-011 CLI-surface enforcement confirmed present

#### Convergence Status
- Consecutive clean passes: 0 of 3 required
- Pass 1: ADJUDICATED-REMEDIATED (D-008 F-02/F-03 accept+defer; D-009 F-04 guard; D-010 F-01 oracle)
- Pass 2: NOT CLEAN (4 MEDIUM findings ESCALATED-PENDING-OPERATOR)

#### Next Steps
ESCALATE-BEFORE-FIX: Present the 4 Pass-2 MEDIUM findings to operator for adjudication (accept-and-fix / defer-tech-debt / reject-out-of-scope). After adjudication: remediate accepted findings, re-run independent CI gate, run Pass 3. 3 consecutive clean passes still required (0/3).

