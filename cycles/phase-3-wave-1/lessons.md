---
document_type: lessons
level: ops
version: "1.0"
status: in-progress
producer: state-manager
timestamp: 2026-08-19T14:24:32Z
cycle: "phase-3-wave-1"
input-hash: "[md5]"
traces_to: STATE.md
---

# Lessons Learned — phase-3-wave-1

## x3 Fmt-Drift Recurrence (S-1.01 F-01+F-04 fix pair)

### Pattern
Test files committed fmt-dirty three times in this engagement:
1. First occurrence: Test files committed with `cargo fmt --all -- --check` failing
2. Second occurrence: Test files committed with `cargo fmt --all -- --check` failing
3. Third occurrence (this cycle): Both 3b705eb (scanner_discovery_tests.rs) and the original F-04 commit (pure_core_guard.rs) failed `cargo fmt --all -- --check` as first committed

### Observation
Subagent gate self-reports (FMT_EXIT=0) are not evidence — orchestrator must run the FULL gate (fmt+clippy+nextest) on every committed fix, never trusting subagent gate self-reports.

### Resolution
- F-01 (3b705eb): fmt-dirty test file corrected in 2859e03 (amend + fmt)
- F-04 (710d09b): fmt-dirty test file corrected in 710d09b (amend for fmt-clean)

### Technical Debt
Codification (a per-story-chain pre-commit/CI fmt gate) is DEFERRED for this engagement per operator directive "no pre-gate factory/tooling work"; carried as tech-debt, target: post-wave-4 factory hardening.

### Interim Mitigation
Orchestrator continues to run the full independent gate (fmt+clippy+nextest) on every committed fix, never trusting subagent gate self-reports.

### Status
RESOLVED for this cycle — fixed in commits 3b705eb → 2859e03 (F-01) and 710d09b (F-04, amended fmt-clean).

---