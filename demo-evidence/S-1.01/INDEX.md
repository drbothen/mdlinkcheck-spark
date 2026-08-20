# S-1.01 Demo Evidence Index

This directory contains visual evidence of acceptance criterion compliance for story **S-1.01: Workspace Scaffold, Shared Types, and Default-CWD File Discovery**.

## Evidence Artifacts

| AC ID | Description | Evidence Files | Test Command |
|-------|-------------|----------------|--------------|
| AC-001 | Default CWD scan includes all .md files | [AC-001.gif](./AC-001.gif), [AC-001.webm](./AC-001.webm) | `test_BC_2_01_001_default_cwd_scan_includes_all_md_files` |
| AC-002 | No duplicate files in scan set | [AC-002.gif](./AC-002.gif), [AC-002.webm](./AC-002.webm) | `test_BC_2_01_001_no_duplicate_in_scan_set` |
| AC-003 | Scan terminates for finite tree | [AC-003.gif](./AC-003.gif), [AC-003.webm](./AC-003.webm) | `test_BC_2_01_001_scan_terminates_for_finite_tree` |
| AC-004 | .gitignore excludes from scan set | [AC-004.gif](./AC-004.gif), [AC-004.webm](./AC-004.webm) | `test_BC_2_01_003_gitignore_excludes_from_scan_set` |
| AC-005 | Gitignored file not scanned as source | [AC-005.gif](./AC-005.gif), [AC-005.webm](./AC-005.webm) | `test_BC_2_01_003_gitignored_file_not_scanned_as_source` |
| AC-006 | Gitignored file anchor table built as target | [AC-006.gif](./AC-006.gif), [AC-006.webm](./AC-006.webm) | `test_BC_2_01_003_gitignored_file_anchor_table_built_as_target` |
| AC-007 | Dot-directories unconditionally skipped | [AC-007.gif](./AC-007.gif), [AC-007.webm](./AC-007.webm) | `test_BC_2_01_004_dot_directories_unconditionally_skipped` |
| AC-008 | No override flag for dot-directory skip | [AC-008.gif](./AC-008.gif), [AC-008.webm](./AC-008.webm) | `test_BC_2_01_004_no_override_flag_for_dot_dir_skip` |
| AC-009 | Directory symlinks not followed | [AC-009.gif](./AC-009.gif), [AC-009.webm](./AC-009.webm) | `test_BC_2_01_004_directory_symlinks_not_followed` |
| AC-010 | Dot-dir MD file anchor table built as target | [AC-010.gif](./AC-010.gif), [AC-010.webm](./AC-010.webm) | `test_BC_2_01_004_dot_dir_md_file_anchor_table_built_as_target` |
| AC-011 | Exact .md extension included | [AC-011.gif](./AC-011.gif), [AC-011.webm](./AC-011.webm) | `test_BC_2_01_005_exact_md_extension_included` |
| AC-012 | Non-.md extensions excluded | [AC-012.gif](./AC-012.gif), [AC-012.webm](./AC-012.webm) | `test_BC_2_01_005_non_md_extensions_excluded` |
| AC-013 | Case-sensitive byte match for .md | [AC-013.gif](./AC-013.gif), [AC-013.webm](./AC-013.webm) | `test_BC_2_01_005_case_sensitive_byte_match` |

## Notes

- **AC-006 and AC-010**: These acceptance criteria have a deferred component (Pass 1.5 anchor table building) per D-008 to BC-2.08.004/SS-05. The demos show only the in-scope "not in scan set" half of each AC. The anchor table building is performed by the `anchor_table` module in Pass 1.5.

## Recording Method

All recordings were generated using [VHS](https://github.com/charmbracelet/vhs) terminal recording tool, demonstrating test execution via `cargo test`. This is honest evidence of the foundational Rust core implementation - there is no user-facing CLI binary yet (the binary's `main.rs` is a stub `todo!()` as documented in S-1.01).

## Test Results Summary

- Total tests run: 34
- Passed: 34
- Failed: 0
- Filtered out: 0

Test output:
```
running 34 tests
test test_BC_2_01_004_directory_symlinks_not_followed ... ok
test test_BC_2_01_004_no_override_flag_for_dot_dir_skip ... ok
test test_BC_2_01_003_gitignored_file_anchor_table_built_as_target ... ok
test test_BC_2_01_003_gitignored_file_not_scanned_as_source ... ok
test test_BC_2_01_003_global_gitignore_out_of_scope ... ok
test test_BC_2_01_004_dot_files_should_be_included ... ok
test test_BC_2_01_001_dot_ancestor_should_not_block_scan ... ok
test test_BC_2_01_001_scan_terminates_for_finite_tree ... ok
test test_BC_2_01_001_default_cwd_scan_includes_all_md_files ... ok
test test_BC_2_01_004_dot_dir_md_file_anchor_table_built_as_target ... ok
test test_BC_2_01_001_scan_terminates_with_genuine_symlink_cycle ... ok
test test_BC_2_01_004_dot_directories_still_skipped_after_dot_file_fix ... ok
test test_BC_2_01_003_nested_gitignore_respected ... ok
test test_BC_2_01_004_dot_directories_unconditionally_skipped ... ok
test test_BC_2_01_003_gitignore_excludes_from_scan_set ... ok
test test_BC_2_01_001_no_duplicate_in_scan_set ... ok
test test_EC_001_empty_tree_no_crash ... ok
test test_BC_2_01_005_case_sensitive_byte_match ... ok
test test_EC_006a_markdown_rejected ... ok
test test_EC_005_uppercase_md_rejected ... ok
test test_F_SCAN_DOT_ROOT_dot_prefixed_root_dir_is_scanned ... ok
test test_EC_008_symlink_cycle_terminates ... ok
test test_EC_009_dir_symlink_not_followed ... ok
test test_EC_006b_mdx_rejected ... ok
test test_EC_003_dot_github_skipped ... ok
test test_EC_004_git_dir_skipped ... ok
test test_boundary_file_with_spaces ... ok
test test_boundary_hidden_files_in_nonhidden_dirs ... ok
test test_BC_2_01_005_exact_md_extension_included ... ok
test test_BC_2_01_005_non_md_extensions_excluded ... ok
test test_mixed_scenarios_gitignore_and_dotdirs ... ok
test test_VP_016_gitignore_patterns_exclude_from_scan_set ... ok
test test_boundary_deep_nested_files ... ok
test test_VP_017_proptest_scan_terminates_for_bounded_tree_with_symlink_cycle ... ok

test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Verification Properties

| VP | Property | Covered By |
|----|----------|------------|
| VP-016 | `.gitignore` exclusion during traversal | AC-004, AC-005, AC-006 |
| VP-017 | Scan terminates for arbitrary directory tree with symlink cycles | AC-003, AC-009, AC-010 |
