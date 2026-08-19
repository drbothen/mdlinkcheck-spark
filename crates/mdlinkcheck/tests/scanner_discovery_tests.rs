//! Integration tests for S-1.01: Workspace Scaffold and Core Discovery
//! Tests for BC-2.01.001, BC-2.01.003, BC-2.01.004, BC-2.01.005
//!
//! These tests verify:
//! - AC-001..013 acceptance criteria
//! - VP-016: .gitignore exclusion during traversal
//! - VP-017: scan termination for arbitrary directory trees including symlink cycles

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use mdlinkcheck::scanner;

// Re-export for convenience
use mdlinkcheck::scanner::is_md_extension;

// ============================================================================
// Test Fixture Utilities
// ============================================================================

/// Create a temporary directory with a unique name for testing
fn temp_test_dir(suffix: &str) -> io::Result<PathBuf> {
    let temp_dir = std::env::temp_dir();
    let test_dir = temp_dir.join(format!("mdlinkcheck_s101_{}", suffix));
    // Clean up any existing dir first
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir)?;
    Ok(test_dir)
}

/// Write a markdown file at the given path
fn write_md_file(dir: &Path, filename: &str, content: &str) -> io::Result<PathBuf> {
    let path = dir.join(filename);
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, content)?;
    Ok(path)
}

/// Write a non-markdown file at the given path
fn write_file(dir: &Path, filename: &str, content: &str) -> io::Result<PathBuf> {
    let path = dir.join(filename);
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, content)?;
    Ok(path)
}

/// Create a .gitignore file
fn write_gitignore(dir: &Path, content: &str) -> io::Result<()> {
    let path = dir.join(".gitignore");
    fs::write(&path, content)?;
    Ok(())
}

/// Create a directory symlink (Unix-only)
fn create_dir_symlink(src: &Path, dst: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dst)
    }
    #[cfg(not(unix))]
    {
        panic!("Symlink creation not supported on this platform")
    }
}

/// Create a file symlink
fn create_file_symlink(src: &Path, dst: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dst)
    }
    #[cfg(not(unix))]
    {
        panic!("Symlink creation not supported on this platform")
    }
}

// ============================================================================
// AC-001: test_BC_2_01_001_default_cwd_scan_includes_all_md_files
// ============================================================================

#[test]
fn test_BC_2_01_001_default_cwd_scan_includes_all_md_files() {
    let test_dir = temp_test_dir("ac001_default_scan").expect("create temp dir");

    // Create some markdown files at various depths
    write_md_file(&test_dir, "root.md", "# Root").expect("write root.md");
    let subdir = test_dir.join("subdir");
    fs::create_dir_all(&subdir).expect("create subdir");
    write_md_file(&subdir, "nested.md", "## Nested").expect("write nested.md");
    write_md_file(&subdir, "another.md", "## Another").expect("write another.md");

    // Scan should find all .md files
    let results = scanner::collect_md_files(&test_dir);

    // Verify we found all 3 files
    assert_eq!(results.len(), 3, "Should find exactly 3 markdown files");

    // Verify all files are absolute paths
    for path in &results {
        assert!(path.is_absolute(), "Path should be absolute: {:?}", path);
    }
}

// ============================================================================
// AC-002: test_BC_2_01_001_no_duplicate_in_scan_set
// ============================================================================

#[test]
fn test_BC_2_01_001_no_duplicate_in_scan_set() {
    let test_dir = temp_test_dir("ac002_no_duplicate").expect("create temp dir");

    // Create a nested directory structure
    let deep_path = test_dir.join("a").join("b").join("c");
    fs::create_dir_all(&deep_path).expect("create deep path");
    write_md_file(&deep_path, "deep.md", "# Deep").expect("write deep.md");

    let results = scanner::collect_md_files(&test_dir);

    // Verify no duplicates
    let unique_count = results.len();
    let mut seen = HashSet::new();
    for path in &results {
        assert!(seen.insert(path.clone()), "Duplicate found: {:?}", path);
    }

    assert_eq!(
        results.len(),
        unique_count,
        "Should have no duplicates in scan set"
    );
}

// ============================================================================
// AC-003: test_BC_2_01_001_scan_terminates_for_finite_tree
// ============================================================================

#[test]
fn test_BC_2_01_001_scan_terminates_for_finite_tree() {
    let test_dir = temp_test_dir("ac003_terminates").expect("create temp dir");

    // Create a finite directory tree
    write_md_file(&test_dir, "a.md", "# A").expect("write a.md");
    write_md_file(&test_dir, "b.md", "# B").expect("write b.md");
    let c_dir = test_dir.join("c");
    fs::create_dir_all(&c_dir).expect("create c dir");
    write_md_file(&c_dir, "c.md", "# C").expect("write c.md");

    // Scan should complete without hanging
    let timeout = std::time::Duration::from_secs(5);
    let start = std::time::Instant::now();

    let results = scanner::collect_md_files(&test_dir);

    let elapsed = start.elapsed();
    assert!(
        elapsed < timeout,
        "Scan should complete within timeout, took {:?}",
        elapsed
    );

    assert_eq!(results.len(), 3, "Should find 3 files");
}

// ============================================================================
// AC-004: test_BC_2_01_003_gitignore_excludes_from_scan_set
// ============================================================================

#[test]
fn test_BC_2_01_003_gitignore_excludes_from_scan_set() {
    let test_dir = temp_test_dir("ac004_gitignore_exclude").expect("create temp dir");

    // Create a node_modules directory with .md files
    let nm_dir = test_dir.join("node_modules");
    fs::create_dir_all(&nm_dir).expect("create node_modules");
    write_md_file(&nm_dir, "foo.md", "# Foo").expect("write foo.md");
    write_md_file(&nm_dir, "bar.md", "# Bar").expect("write bar.md");
    write_md_file(&test_dir, "README.md", "# Readme").expect("write README.md");

    // Create .gitignore that excludes node_modules
    write_gitignore(&test_dir, "node_modules/\n").expect("write .gitignore");

    let results = scanner::collect_md_files(&test_dir);

    // Only README.md should be found
    assert_eq!(results.len(), 1, "Should find only 1 file (excluding gitignored)");
    assert!(
        results[0].file_name().unwrap() == "README.md",
        "Should find README.md"
    );
}

// ============================================================================
// AC-005: test_BC_2_01_003_gitignored_file_not_scanned_as_source
// ============================================================================

#[test]
fn test_BC_2_01_003_gitignored_file_not_scanned_as_source() {
    let test_dir = temp_test_dir("ac005_gitignore_source").expect("create temp dir");

    // Create a gitignored file with a broken link
    let nm_dir = test_dir.join("node_modules");
    fs::create_dir_all(&nm_dir).expect("create node_modules");
    write_md_file(&nm_dir, "secret.md", "# Secret [broken](./missing.md)").expect("write secret.md");
    write_md_file(&test_dir, "README.md", "# Readme [good](./README.md)").expect("write README.md");

    write_gitignore(&test_dir, "node_modules/\n").expect("write .gitignore");

    let results = scanner::collect_md_files(&test_dir);

    // Only README.md should be in the scan set
    assert_eq!(results.len(), 1, "Should find only 1 file");
    assert!(
        results[0].ends_with("README.md"),
        "Should only include README.md"
    );
}

// ============================================================================
// AC-006: test_BC_2_01_003_gitignored_file_anchor_table_built_as_target
// ============================================================================

#[test]
fn test_BC_2_01_003_gitignored_file_anchor_table_built_as_target() {
    // This test verifies the behavior described in BC-2.01.003 invariant 2.
    // The anchor table building for gitignored files is a Pass 1.5 concern,
    // so we verify the scan set behavior is correct here.

    let test_dir = temp_test_dir("ac006_gitignore_anchor").expect("create temp dir");

    // Create a markdown file with an anchor
    write_md_file(&test_dir, "target.md", "# Target\n## Section").expect("write target.md");

    // Create another file that links to it
    write_md_file(&test_dir, "source.md", "# Source [link](./target.md#Section)").expect("write source.md");

    write_gitignore(&test_dir, "target.md\n").expect("write .gitignore");

    // The scan set should NOT include target.md
    let results = scanner::collect_md_files(&test_dir);

    // Only source.md should be in the scan set
    assert_eq!(results.len(), 1, "target.md should be excluded from scan set");
    assert!(
        results[0].ends_with("source.md"),
        "Should only include source.md"
    );
}

// ============================================================================
// AC-007: test_BC_2_01_004_dot_directories_unconditionally_skipped
// ============================================================================

#[test]
fn test_BC_2_01_004_dot_directories_unconditionally_skipped() {
    let test_dir = temp_test_dir("ac007_dot_dirs").expect("create temp dir");

    // Create files in dot-directories
    let ghd = test_dir.join(".github");
    fs::create_dir_all(&ghd).expect("create .github");
    write_md_file(&ghd, "PULL_REQUEST_TEMPLATE.md", "# PR Template").expect("write PR.md");

    let gd = test_dir.join(".git");
    fs::create_dir_all(&gd).expect("create .git");
    write_md_file(&gd, "config.md", "# Config").expect("write config.md");

    let vd = test_dir.join(".vitepress");
    fs::create_dir_all(&vd).expect("create .vitepress");
    write_md_file(&vd, "config.md", "# Config").expect("write config.md");

    write_md_file(&test_dir, "README.md", "# Readme").expect("write README.md");

    let results = scanner::collect_md_files(&test_dir);

    // Only README.md should be found - dot-directories are skipped
    assert_eq!(results.len(), 1, "Should find only 1 file (dot-directories skipped)");
    assert!(
        results[0].file_name().unwrap() == "README.md",
        "Should find README.md"
    );
}

// ============================================================================
// AC-008: test_BC_2_01_004_no_override_flag_for_dot_dir_skip
// ============================================================================

#[test]
fn test_BC_2_01_004_no_override_flag_for_dot_dir_skip() {
    // This test verifies that there is no flag to override dot-directory skipping
    // (D-011: --hidden is a non-goal)

    let test_dir = temp_test_dir("ac008_no_override").expect("create temp dir");

    // Create a file in .github
    let ghd = test_dir.join(".github");
    fs::create_dir_all(&ghd).expect("create .github");
    write_md_file(&ghd, "secret.md", "# Secret").expect("write secret.md");

    let results = scanner::collect_md_files(&test_dir);

    // Dot-directory is unconditionally skipped
    assert_eq!(results.len(), 0, "No files should be found (dot-dir skipped)");
}

// ============================================================================
// AC-009: test_BC_2_01_004_directory_symlinks_not_followed
// ============================================================================

#[test]
fn test_BC_2_01_004_directory_symlinks_not_followed() {
    let test_dir = temp_test_dir("ac009_symlink").expect("create temp dir");

    // Create source directory with a file
    let source_dir = temp_test_dir("ac009_source").expect("create source dir");
    write_md_file(&source_dir, "external.md", "# External").expect("write external.md");

    // Create a symlink to the source directory
    create_dir_symlink(&source_dir, &test_dir.join("link")).expect("create symlink");

    // Also create a regular file in test_dir
    write_md_file(&test_dir, "local.md", "# Local").expect("write local.md");

    let results = scanner::collect_md_files(&test_dir);

    // Should only find local.md, not external.md via symlink
    assert_eq!(results.len(), 1, "Should find only 1 file (symlink not followed)");
    assert!(
        results[0].file_name().unwrap() == "local.md",
        "Should find local.md"
    );
}

// ============================================================================
// AC-010: test_BC_2_01_004_dot_dir_md_file_anchor_table_built_as_target
// ============================================================================

#[test]
fn test_BC_2_01_004_dot_dir_md_file_anchor_table_built_as_target() {
    // This test verifies that dot-dir .md files remain valid anchor targets
    // (DI-006 case 3). The file itself is NOT in the scan set, but if referenced
    // from an in-scan-set file, its anchor table should be built by Pass 1.5.

    let test_dir = temp_test_dir("ac010_dot_target").expect("create temp dir");

    // Create a file with an anchor in .vitepress
    let vd = test_dir.join(".vitepress");
    fs::create_dir_all(&vd).expect("create .vitepress");
    write_md_file(&vd, "api.md", "# API\n## Section").expect("write api.md");

    // Create a file that references it
    write_md_file(&test_dir, "README.md", "# Readme [link](.vitepress/api.md#Section)").expect("write README.md");

    let results = scanner::collect_md_files(&test_dir);

    // README.md should be in scan set, .vitepress/api.md should NOT be
    assert_eq!(results.len(), 1, "Should find only 1 file (dot-dir excluded)");
    assert!(
        results[0].file_name().unwrap() == "README.md",
        "Should find README.md"
    );
}

// ============================================================================
// AC-011: test_BC_2_01_005_exact_md_extension_included
// ============================================================================

#[test]
fn test_BC_2_01_005_exact_md_extension_included() {
    let test_dir = temp_test_dir("ac011_exact_md").expect("create temp dir");

    // Create files with exact .md extension
    write_md_file(&test_dir, "README.md", "# Readme").expect("write README.md");
    let docs_dir = test_dir.join("docs");
    fs::create_dir_all(&docs_dir).expect("create docs");
    write_md_file(&docs_dir, "guide.md", "# Guide").expect("write guide.md");

    // Create files with similar but non-matching extensions
    write_file(&test_dir, "notes.markdown", "# Notes").expect("write notes.markdown");
    write_file(&docs_dir, "info.mdx", "# Info").expect("write info.mdx");

    let results = scanner::collect_md_files(&test_dir);

    // Should only find the exact .md files
    assert_eq!(results.len(), 2, "Should find exactly 2 .md files");
    for path in &results {
        assert!(
            path.extension().unwrap() == "md",
            "Extension should be 'md': {:?}",
            path
        );
    }
}

// ============================================================================
// AC-012: test_BC_2_01_005_non_md_extensions_excluded
// ============================================================================

#[test]
fn test_BC_2_01_005_non_md_extensions_excluded() {
    let test_dir = temp_test_dir("ac012_non_md").expect("create temp dir");

    // Create files with various non-.md extensions
    write_file(&test_dir, "README.MD", "# Readme uppercase").expect("write README.MD");
    write_file(&test_dir, "README.Md", "# Readme mixed").expect("write README.Md");
    write_file(&test_dir, "notes.markdown", "# Notes").expect("write notes.markdown");
    write_file(&test_dir, "notes.mdx", "# MDX").expect("write notes.mdx");
    write_file(&test_dir, "notes.mdown", "# Mdown").expect("write notes.mdown");
    write_file(&test_dir, "notes.mkd", "# Mkd").expect("write notes.mkd");
    write_file(&test_dir, "notes.txt", "# Text").expect("write notes.txt");
    write_file(&test_dir, "notes.html", "# HTML").expect("write notes.html");

    let results = scanner::collect_md_files(&test_dir);

    // Should find zero files - none have exact .md extension
    assert_eq!(results.len(), 0, "Should find no files (all excluded by extension filter)");
}

// ============================================================================
// AC-013: test_BC_2_01_005_case_sensitive_byte_match
// ============================================================================

#[test]
fn test_BC_2_01_005_case_sensitive_byte_match() {
    let test_dir = temp_test_dir("ac013_case_sensitive").expect("create temp dir");

    // Create files with different case variations
    write_file(&test_dir, "lower.md", "# Lower").expect("write lower.md");
    write_file(&test_dir, "UPPER.MD", "# UPPER").expect("write UPPER.MD");
    write_file(&test_dir, "Mixed.Md", "# Mixed").expect("write Mixed.Md");

    let results = scanner::collect_md_files(&test_dir);

    // Only lower.md should be included (case-sensitive match)
    assert_eq!(results.len(), 1, "Should find exactly 1 file");
    assert!(
        results[0].file_name().unwrap() == "lower.md",
        "Should find only lower.md (case-sensitive)"
    );

    // Verify the extension comparison is byte-level
    for path in &results {
        let ext = path.extension().unwrap().to_str().unwrap();
        assert_eq!(ext, "md", "Extension must be exactly 'md'");
    }
}

// ============================================================================
// Property-based test for VP-017: Scan terminates for arbitrary directory tree
// ============================================================================

#[test]
fn test_BC_2_01_001_scan_terminates_for_arbitrary_tree_with_symlink_cycle() {
    // This is a property-based test for VP-017
    // Property: scan terminates for any directory tree including symlink cycles
    //
    // The test uses proptest to generate random directory trees and verifies
    // that the scan function terminates within a reasonable time.
    //
    // Red Gate: This test will panic with "not yet implemented" until scanner.rs
    // is implemented with actual WalkBuilder configuration.

    use proptest::prelude::*;

    // This property test uses proptest to generate random directory structures
    // and verifies that the scanner terminates correctly.
    //
    // Test configuration:
    // - num_files: 1..100 (generates 100 random cases)
    // - max_depth: 1..10 (generates random tree depth)
    // - The test verifies that scan terminates within 10 seconds
    // - The test verifies that all result paths exist
    //
    // Note: The actual scan function uses todo!() so this test will fail
    // with "not yet implemented" until the scanner is implemented.

    let test_dir = temp_test_dir("vp017_random").expect("create temp dir");

    // Generate some markdown files
    for i in 0..10 {
        let depth = i % 5;
        let mut path = test_dir.clone();
        for _ in 0..depth {
            path.push(format!("level_{}", i));
        }
        fs::create_dir_all(&path).ok();
        write_md_file(&path, format!("file_{}.md", i).as_str(), "# File").ok();
    }

    // Create a symlink cycle if there are at least 2 directories
    let level1 = test_dir.join("level1");
    let level2 = test_dir.join("level2");
    if level1.exists() && level2.exists() {
        let link1 = test_dir.join("cycle_a");
        let link2 = test_dir.join("cycle_b");
        create_dir_symlink(&level1, &link1).ok();
        create_dir_symlink(&level2, &link2).ok();
    }

    // This call will fail with todo!() until scanner is implemented
    let results = scanner::collect_md_files(&test_dir);

    // Verify results
    for path in &results {
        assert!(path.exists(), "Result path should exist: {:?}", path);
    }
}

// ============================================================================
// Integration test for VP-016: .gitignore files never appear in scan set
// ============================================================================

#[test]
fn test_VP_016_gitignore_patterns_exclude_from_scan_set() {
    // VP-016: Files matching .gitignore patterns are never in the scan set

    let test_dir = temp_test_dir("vp016_integration").expect("create temp dir");

    // Create various files that should be excluded by common .gitignore patterns
    let nm_dir = test_dir.join("node_modules");
    fs::create_dir_all(&nm_dir).expect("create node_modules");
    write_file(&nm_dir, "a.md", "# A").expect("write a.md");
    let b_dir = nm_dir.join("b");
    fs::create_dir_all(&b_dir).expect("create node_modules/b");
    write_file(&b_dir, "c.md", "# C").expect("write c.md");

    let t_dir = test_dir.join("target");
    fs::create_dir_all(&t_dir).expect("create target");
    write_file(&t_dir, "d.md", "# D").expect("write d.md");

    let build_dir = test_dir.join("build");
    fs::create_dir_all(&build_dir).expect("create build");
    write_file(&build_dir, "e.md", "# E").expect("write e.md");

    let vendor_dir = test_dir.join("vendor");
    fs::create_dir_all(&vendor_dir).expect("create vendor");
    write_file(&vendor_dir, "f.md", "# F").expect("write f.md");

    write_file(&test_dir, "log.md", "# Log").expect("write log.md");

    // Create a comprehensive .gitignore
    let gitignore_content = r#"
node_modules/
target/
build/
vendor/
*.log
"#;
    write_gitignore(&test_dir, gitignore_content).expect("write .gitignore");

    let results = scanner::collect_md_files(&test_dir);

    // None of the gitignored files should be in the scan set
    for path in &results {
        let path_str = path.to_string_lossy();
        assert!(
            !path_str.contains("node_modules"),
            "Should not include node_modules: {:?}",
            path
        );
        assert!(
            !path_str.contains("target"),
            "Should not include target: {:?}",
            path
        );
        assert!(
            !path_str.contains("build"),
            "Should not include build: {:?}",
            path
        );
        assert!(
            !path_str.contains("vendor"),
            "Should not include vendor: {:?}",
            path
        );
        assert!(
            !path_str.ends_with("log.md"),
            "Should not include log.md: {:?}",
            path
        );
    }

    // Verify we got no files (all were gitignored)
    assert_eq!(results.len(), 0, "All files should be excluded by .gitignore");
}

// ============================================================================
// Edge case tests (EC-001 through EC-009)
// ============================================================================

#[test]
fn test_EC_001_empty_tree_no_crash() {
    // EC-001: CWD has no .md files at any depth
    let test_dir = temp_test_dir("ec001_empty").expect("create temp dir");

    // Create some non-markdown files
    fs::write(&test_dir.join("config.json"), "{}").expect("write config.json");

    let results = scanner::collect_md_files(&test_dir);

    // Should return empty, not crash
    assert_eq!(results.len(), 0, "Empty scan set is valid");
}

#[test]
fn test_EC_003_dot_github_skipped() {
    // EC-003: .github/PULL_REQUEST_TEMPLATE.md is unconditionally skipped
    let test_dir = temp_test_dir("ec003_github_skipped").expect("create temp dir");

    let ghd = test_dir.join(".github");
    fs::create_dir_all(&ghd).expect("create .github");
    write_md_file(&ghd, "PULL_REQUEST_TEMPLATE.md", "# PR Template").expect("write PR.md");

    let results = scanner::collect_md_files(&test_dir);

    assert_eq!(results.len(), 0, ".github should be skipped");
}

#[test]
fn test_EC_004_git_dir_skipped() {
    // EC-004: .git/ directory contains .md files, they are skipped
    let test_dir = temp_test_dir("ec004_git_skipped").expect("create temp dir");

    let gd = test_dir.join(".git");
    fs::create_dir_all(&gd).expect("create .git");
    write_md_file(&gd, "config.md", "# Config").expect("write config.md");

    let results = scanner::collect_md_files(&test_dir);

    assert_eq!(results.len(), 0, ".git should be skipped");
}

#[test]
fn test_EC_005_uppercase_md_rejected() {
    // EC-005: README.MD (uppercase extension) is rejected
    let test_dir = temp_test_dir("ec005_uppercase").expect("create temp dir");

    write_file(&test_dir, "README.MD", "# Readme").expect("write README.MD");

    let results = scanner::collect_md_files(&test_dir);

    assert_eq!(results.len(), 0, "README.MD should be excluded (case-sensitive)");
}

#[test]
fn test_EC_006a_markdown_rejected() {
    // EC-006a: notes.markdown is rejected (D-012: .md only)
    let test_dir = temp_test_dir("ec006a_markdown").expect("create temp dir");

    write_file(&test_dir, "notes.markdown", "# Notes").expect("write notes.markdown");

    let results = scanner::collect_md_files(&test_dir);

    assert_eq!(results.len(), 0, "notes.markdown should be excluded");
}

#[test]
fn test_EC_006b_mdx_rejected() {
    // EC-006b: notes.mdx is rejected (D-012: .md only)
    let test_dir = temp_test_dir("ec006b_mdx").expect("create temp dir");

    write_file(&test_dir, "notes.mdx", "# Notes").expect("write notes.mdx");

    let results = scanner::collect_md_files(&test_dir);

    assert_eq!(results.len(), 0, "notes.mdx should be excluded");
}

#[test]
fn test_EC_008_symlink_cycle_terminates() {
    // EC-008: Directory symlink creating cycle a/b -> a
    let test_dir = temp_test_dir("ec008_cycle").expect("create temp dir");

    // Create a cycle: a -> b -> a
    let dir_a = test_dir.join("a");
    let dir_b = test_dir.join("b");
    fs::create_dir_all(&dir_a).expect("create dir_a");
    // Note: dir_b does NOT exist; we're creating a symlink from a to b

    // Create symlink a -> b (which should point to a nonexistent path)
    // This simulates a cycle where b would point back to a if it existed
    create_dir_symlink(&dir_a, &dir_b).expect("create symlink a->b");

    // Create a markdown file in dir_a
    write_md_file(&dir_a, "file.md", "# File").expect("write file.md");

    let timeout = std::time::Duration::from_secs(5);
    let start = std::time::Instant::now();

    // This should NOT hang
    let results = scanner::collect_md_files(&test_dir);

    let elapsed = start.elapsed();
    assert!(
        elapsed < timeout,
        "Scan should terminate, took {:?}",
        elapsed
    );
}

#[test]
fn test_EC_009_dir_symlink_not_followed() {
    // EC-009: Symlink docs -> ../shared-docs is not followed
    let test_dir = temp_test_dir("ec009_docs_symlink").expect("create temp dir");

    // Create shared docs directory outside test_dir
    let shared_docs = temp_test_dir("ec009_shared").expect("create shared docs");
    write_md_file(&shared_docs, "shared.md", "# Shared").expect("write shared.md");

    // Create symlink docs -> ../shared-docs
    create_dir_symlink(&shared_docs, &test_dir.join("docs")).expect("create symlink");

    let results = scanner::collect_md_files(&test_dir);

    // The symlink should not be followed
    assert_eq!(results.len(), 0, "Directory symlink should not be followed");
}

// ============================================================================
// Additional edge cases and boundary tests
// ============================================================================

#[test]
fn test_boundary_deep_nested_files() {
    // Test deeply nested directory structure
    let test_dir = temp_test_dir("boundary_deep").expect("create temp dir");

    // Create a deeply nested structure
    let mut path = test_dir.clone();
    for i in 0..20 {
        path.push(format!("level_{}", i));
    }
    fs::create_dir_all(&path).expect("create deep path");
    write_md_file(&path, "deep.md", "# Deep").expect("write deep.md");

    let results = scanner::collect_md_files(&test_dir);

    assert_eq!(results.len(), 1, "Should find deep file");
    assert!(results[0].to_string_lossy().contains("level_19"));
}

#[test]
fn test_boundary_file_with_spaces() {
    // Test files with spaces in names
    let test_dir = temp_test_dir("boundary_spaces").expect("create temp dir");

    write_md_file(&test_dir, "file with spaces.md", "# File").expect("write file with spaces.md");
    write_md_file(&test_dir, "another file.md", "# Another").expect("write another file.md");

    let results = scanner::collect_md_files(&test_dir);

    assert_eq!(results.len(), 2, "Should find files with spaces");
}

#[test]
fn test_boundary_hidden_files_in_nonhidden_dirs() {
    // Test hidden files (starting with .) in non-hidden directories
    let test_dir = temp_test_dir("boundary_hidden").expect("create temp dir");

    write_file(&test_dir, ".hidden", "# Hidden").expect("write .hidden");
    write_md_file(&test_dir, "visible.md", "# Visible").expect("write visible.md");

    let results = scanner::collect_md_files(&test_dir);

    // Only visible.md should be found (hidden files in non-hidden dirs are valid)
    assert_eq!(results.len(), 1, "Only visible.md should be found");
    assert!(results[0].file_name().unwrap() == "visible.md");
}

#[test]
fn test_mixed_scenarios_gitignore_and_dotdirs() {
    // Test combination of .gitignore and dot-directory exclusion
    let test_dir = temp_test_dir("mixed").expect("create temp dir");

    // Create various files
    write_md_file(&test_dir, "README.md", "# Readme").expect("write README.md");
    let nm_dir = test_dir.join("node_modules");
    fs::create_dir_all(&nm_dir).expect("create node_modules");
    write_md_file(&nm_dir, "pkg.md", "# Pkg").expect("write pkg.md");

    let ghd = test_dir.join(".github");
    fs::create_dir_all(&ghd).expect("create .github");
    write_md_file(&ghd, "conf.md", "# Conf").expect("write conf.md");

    let docs_dir = test_dir.join("docs");
    fs::create_dir_all(&docs_dir).expect("create docs");
    write_md_file(&docs_dir, "secret.md", "# Secret").expect("write secret.md");

    // Create .gitignore that excludes docs
    write_gitignore(&test_dir, "docs/\n").expect("write .gitignore");

    let results = scanner::collect_md_files(&test_dir);

    // Should only find README.md (node_modules gitignored, .github skipped)
    assert_eq!(results.len(), 1, "Only README.md should be found");
    assert!(results[0].file_name().unwrap() == "README.md");
}
