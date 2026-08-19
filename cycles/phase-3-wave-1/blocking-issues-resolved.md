---
document_type: blocking-issues-resolved
level: ops
version: "1.0"
status: in-progress
producer: state-manager
timestamp: 2026-08-19T05:55:00Z
cycle: "phase-3-wave-1"
input-hash: "[md5]"
traces_to: STATE.md
---

# Resolved Blocking Issues — phase-3-wave-1

## F1 (BC-2.01.004 PC3 file-symlink following) — DEFERRED

| Field | Value |
|-------|-------|
| **ID** | F1 |
| **Severity** | HIGH |
| **Category** | spec-vs-impl + coverage |
| **Issue** | BC-2.01.004 PC3 file-symlink following VIOLATED. scanner.rs:24 follow_links(false)+is_file() excludes symlink-to-file. Code comment scanner.rs:52-53 falsely claims PC3 compliance. No AC covers PC3. |
| **VP-INDEX Reference** | 144 |
| **Resolution** | DEFERRED to BC-2.01.006 (separate story) per operator ruling. |
| **Resolution Date** | 2026-08-19 |
| **Fix Required** | Correct the FALSE code comment at scanner.rs:52-53 that claims PC3 compliance, and record a documented coverage-gap / tech-debt deferral entry (PC3 deferred to BC-2.01.006). |
| **Status** | DEFERRED-PENDING (fix wave NOT yet dispatched) |

## F2b (leaf dot-FILE with .md excluded) — PENDING

| Field | Value |
|-------|-------|
| **ID** | F2b |
| **Severity** | HIGH/MEDIUM |
| **Category** | MATERIAL-product-intent |
| **Issue** | Dot-FILE .env.md excluded despite code comment. Root .env.md + normal.md → returned ["normal.md"], .env.md absent, contradicting scanner.rs:27 comment. |
| **Resolution** | INCLUDE dot-files per operator ruling. |
| **Resolution Date** | 2026-08-19 |
| **Fix Required** | Modify filter_entry to skip only dot-DIRECTORIES encountered below the root and include dot-FILES like .env.md. |
| **Status** | PENDING (implementer) |

## OBS-1 (VP-016 label drift) — PARKED

| Field | Value |
|-------|-------|
| **ID** | OBS-1 |
| **Severity** | MEDIUM |
| **Category** | spec-governance |
| **Issue** | VP-016 label drift: VP-INDEX:75,96 define VP-016 = "excluded files are valid anchor targets" (DI-006, anchor_table). But story S-1.01 (lines 71-73) and BC-2.01.003 gloss VP-016 as ".gitignore exclusion / never in scan set". |
| **Resolution** | Specs FROZEN; no self-fix. Escalated to operator as spec-governance FYI. |
| **Resolution Date** | 2026-08-19 |
| **Status** | PARKED (escalated, no self-fix) |

---
