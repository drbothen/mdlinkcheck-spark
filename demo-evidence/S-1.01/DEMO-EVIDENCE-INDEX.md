# S-1.01 Demo Evidence Index

This directory contains visual evidence of acceptance criterion compliance for story **S-1.01: Workspace Scaffold, Shared Types, and Default-CWD File Discovery**.

## Evidence Summary

| AC ID | Traces To | Evidence Files | Test Name | Status |
|-------|-----------|----------------|-----------|--------|
| AC-001 | BC-2.01.001 | [AC-001.gif](./AC-001.gif) [AC-001.webm](./AC-001.webm) | `test_BC_2_01_001_default_cwd_scan_includes_all_md_files` | PASSED |
| AC-002 | BC-2.01.001 | [AC-002.gif](./AC-002.gif) [AC-002.webm](./AC-002.webm) | `test_BC_2_01_001_no_duplicate_in_scan_set` | PASSED |
| AC-003 | BC-2.01.001 | [AC-003.gif](./AC-003.gif) [AC-003.webm](./AC-003.webm) | `test_BC_2_01_001_scan_terminates_for_finite_tree` | PASSED |
| AC-004 | BC-2.01.003 | [AC-004.gif](./AC-004.gif) [AC-004.webm](./AC-004.webm) | `test_BC_2_01_003_gitignore_excludes_from_scan_set` | PASSED |
| AC-005 | BC-2.01.003 | [AC-005.gif](./AC-005.gif) [AC-005.webm](./AC-005.webm) | `test_BC_2_01_003_gitignored_file_not_scanned_as_source` | PASSED |
| AC-006 | BC-2.01.003 | [AC-006.gif](./AC-006.gif) [AC-006.webm](./AC-006.webm) | `test_BC_2_01_003_gitignored_file_anchor_table_built_as_target` | PARTIAL* |
| AC-007 | BC-2.01.004 | [AC-007.gif](./AC-007.gif) [AC-007.webm](./AC-007.webm) | `test_BC_2_01_004_dot_directories_unconditionally_skipped` | PASSED |
| AC-008 | BC-2.01.004 | [AC-008.gif](./AC-008.gif) [AC-008.webm](./AC-008.webm) | `test_BC_2_01_004_no_override_flag_for_dot_dir_skip` | PASSED |
| AC-009 | BC-2.01.004 | [AC-009.gif](./AC-009.gif) [AC-009.webm](./AC-009.webm) | `test_BC_2_01_004_directory_symlinks_not_followed` | PASSED |
| AC-010 | BC-2.01.004 | [AC-010.gif](./AC-010.gif) [AC-010.webm](./AC-010.webm) | `test_BC_2_01_004_dot_dir_md_file_anchor_table_built_as_target` | PARTIAL* |
| AC-011 | BC-2.01.005 | [AC-011.gif](./AC-011.gif) [AC-011.webm](./AC-011.webm) | `test_BC_2_01_005_exact_md_extension_included` | PASSED |
| AC-012 | BC-2.01.005 | [AC-012.gif](./AC-012.gif) [AC-012.webm](./AC-012.webm) | `test_BC_2_01_005_non_md_extensions_excluded` | PASSED |
| AC-013 | BC-2.01.005 | [AC-013.gif](./AC-013.gif) [AC-013.webm](./AC-013.webm) | `test_BC_2_01_005_case_sensitive_byte_match` | PASSED |

\* **PARTIAL** - See notes below

## Notes

### AC-006 and AC-010 Deferred Components

These acceptance criteria have a deferred component (Pass 1.5 anchor table building) per D-008 to BC-2.08.004/SS-05. The demos show only the in-scope "not in scan set" half of each AC. The anchor table building is performed by the `anchor_table` module in Pass 1.5.

- **AC-006 (DI-006 case 2):** Gitignored file that is referenced as an anchor target still has its anchor table built by Pass 1.5.
- **AC-010 (DI-006 case 3):** `.md` file inside a dot-directory that is referenced as an anchor target still has its anchor table built by Pass 1.5.

### Recording Method

All recordings were generated using [VHS](https://github.com/charmbracelet/vhs) terminal recording tool, demonstrating test execution via `cargo test`. This is honest evidence of the foundational Rust core implementation - there is no user-facing CLI binary yet (the binary's `main.rs` is a stub `todo!()` as documented in S-1.01).

### Test Results Summary

- **Total tests run:** 34
- **Passed:** 34
- **Failed:** 0
- **Filtered out:** 0

## Verification Properties

| VP | Property | Covered By |
|----|----------|------------|
| VP-016 | `.gitignore` exclusion during traversal | AC-004, AC-005, AC-006 |
| VP-017 | Scan terminates for arbitrary directory tree with symlink cycles | AC-003, AC-009, AC-010 |

---

*Generated: 2026-08-20*  
*Story: S-1.01 (Workspace Scaffold and Core Discovery)*  
*Branch: feature/S-1.01-workspace-scaffold-and-core-discovery*
