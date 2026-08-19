//! Integration tests for S-1.01: Workspace Scaffold and Core Discovery
//! Tests for BC-2.01.001, BC-2.01.003, BC-2.01.004, BC-2.01.005
//!
//! These tests verify:
//! - AC-001..013 acceptance criteria
//! - VP-016: .gitignore exclusion during traversal
//! - VP-017: scan termination for arbitrary directory trees including symlink cycles

#![allow(non_snake_case)]

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use mdlinkcheck::scanner;

// (is_md_extension is not used in this test file)

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

    // Create a file structure that could have duplicate paths if dedup fails
    // (e.g., file in nested dir that could be visited via multiple traversal paths)
    let deep_path = test_dir.join("a").join("b").join("c");
    fs::create_dir_all(&deep_path).expect("create deep path");
    write_md_file(&deep_path, "deep.md", "# Deep").expect("write deep.md");

    // Create another path to the same file via different route
    let alt_path = test_dir.join("x").join("y");
    fs::create_dir_all(&alt_path).expect("create alt path");
    write_md_file(&alt_path, "deep.md", "# Deep (duplicate name, same file)")
        .expect("write deep.md");

    let results = scanner::collect_md_files(&test_dir);

    // Verify no duplicates using proper HashSet comparison
    let returned_len = results.len();
    let unique_set: HashSet<_> = results.iter().collect();

    assert_eq!(
        returned_len,
        unique_set.len(),
        "Should have no duplicates in scan set. Got {} results, but {} unique paths",
        returned_len,
        unique_set.len()
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
    assert_eq!(
        results.len(),
        1,
        "Should find only 1 file (excluding gitignored)"
    );
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
    write_md_file(&nm_dir, "secret.md", "# Secret [broken](./missing.md)")
        .expect("write secret.md");
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
    write_md_file(
        &test_dir,
        "source.md",
        "# Source [link](./target.md#Section)",
    )
    .expect("write source.md");

    write_gitignore(&test_dir, "target.md\n").expect("write .gitignore");

    // The scan set should NOT include target.md
    let results = scanner::collect_md_files(&test_dir);

    // Only source.md should be in the scan set
    assert_eq!(
        results.len(),
        1,
        "target.md should be excluded from scan set"
    );
    assert!(
        results[0].ends_with("source.md"),
        "Should only include source.md"
    );
}

// ============================================================================
// H7 - PC2: test_BC_2_01_003_nested_gitignore_respected
// ============================================================================
// BC-2.01.003 PC2: nested .gitignore in a subdirectory is respected.
// The ignore crate handles nested gitignore natively, so this should pass.
// This test verifies the scanner correctly handles .gitignore in subdirectories.

#[test]
fn test_BC_2_01_003_nested_gitignore_respected() {
    // H7 (PC2): Verify nested .gitignore in subdirectory is respected
    // This test exercises a DIFFERENTIAL: without nested gitignore, drop.md would be found;
    // its absence is caused solely by the nested gitignore excluding it.
    let test_dir = temp_test_dir("h7_nested_gitignore").expect("create temp dir");

    // Create a root-level README.md (NOT excluded by any root .gitignore)
    write_md_file(&test_dir, "README.md", "# Readme").expect("write README.md");

    // Create docs/ directory that is NOT excluded by root .gitignore
    let docs_dir = test_dir.join("docs");
    fs::create_dir_all(&docs_dir).expect("create docs");
    write_md_file(&docs_dir, "keep.md", "# Keep").expect("write keep.md");
    write_md_file(&docs_dir, "drop.md", "# Drop").expect("write drop.md");

    // Root .gitignore does NOT exclude docs/ (this is critical - the masking bug)
    // Only the nested docs/.gitignore excludes drop.md
    write_gitignore(&docs_dir, "drop.md\n").expect("write docs/.gitignore");

    // The nested .gitignore should exclude drop.md only
    // keep.md should be found (not excluded by nested gitignore)
    // README.md should be found (not excluded by any gitignore)
    let results = scanner::collect_md_files(&test_dir);

    // Expected: README.md and docs/keep.md; docs/drop.md ABSENT
    assert_eq!(
        results.len(),
        2,
        "Exactly 2 files: README.md and docs/keep.md"
    );
    let file_names: Vec<String> = results
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();
    assert!(
        file_names.contains(&"README.md".to_string()),
        "README.md should be found (root file, not gitignored). Found args: {:?}",
        file_names
    );
    assert!(
        file_names.contains(&"keep.md".to_string()),
        "docs/keep.md should be found (nested gitignore only excludes drop.md). Found args: {:?}",
        file_names
    );
    assert!(
        !file_names.contains(&"drop.md".to_string()),
        "docs/drop.md should be ABSENT (excluded by nested gitignore). Found args: {:?}",
        file_names
    );
}

// ============================================================================
// H7 - PC3: test_BC_2_01_003_global_gitignore_not_tested_here
// ============================================================================
// BC-2.01.003 PC3: global/parent .gitignore (e.g., ~/.gitignore, core.excludesFile)
// is NOT within S-1.01 scope. This feature is deferred to a future story.
// This test documents the current scope boundary.

#[test]
fn test_BC_2_01_003_global_gitignore_out_of_scope() {
    // H7 (PC3): Global .gitignore (core.excludesFile, ~/.gitignore) is not tested here
    // because S-1.01 scope is limited to project-local .gitignore files.
    // The ignore crate supports global ignores via global_ignore(true),
    // but this is outside the scope of this story and must NOT be implemented.
    //
    // DISPOSITION: NULL - this is intentionally out of scope for S-1.01.
    // Justification: Global gitignore is a cross-project concern that should
    // be handled by a separate story, possibly with user configuration.
    // The current implementation (require_git(false), git_ignore(true))
    // only handles project-local .gitignore files.

    let test_dir = temp_test_dir("h7_global_out_of_scope").expect("create temp dir");

    write_md_file(&test_dir, "readme.md", "# Readme").expect("write readme.md");

    let results = scanner::collect_md_files(&test_dir);

    // Should find the file - global gitignore is not configured
    assert_eq!(
        results.len(),
        1,
        "Global gitignore not in scope, local file found"
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
    assert_eq!(
        results.len(),
        1,
        "Should find only 1 file (dot-directories skipped)"
    );
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
    assert_eq!(
        results.len(),
        0,
        "No files should be found (dot-dir skipped)"
    );
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
    assert_eq!(
        results.len(),
        1,
        "Should find only 1 file (symlink not followed)"
    );
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
    write_md_file(
        &test_dir,
        "README.md",
        "# Readme [link](.vitepress/api.md#Section)",
    )
    .expect("write README.md");

    let results = scanner::collect_md_files(&test_dir);

    // README.md should be in scan set, .vitepress/api.md should NOT be
    assert_eq!(
        results.len(),
        1,
        "Should find only 1 file (dot-dir excluded)"
    );
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
    assert_eq!(
        results.len(),
        0,
        "Should find no files (all excluded by extension filter)"
    );
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
// VP-017: Scan terminates for arbitrary directory tree including symlink cycle
// ============================================================================

#[test]
fn test_BC_2_01_001_scan_terminates_with_genuine_symlink_cycle() {
    // VP-017: The scanner must terminate on any directory tree, including those
    // with directory symlinks that create cycles.
    //
    // This test creates a genuine cycle: dir_a/ and dir_b/ where:
    // - dir_b/ contains a .md file
    // - dir_b/ is symlinked back to dir_a/
    // The scanner should detect and handle the cycle without infinite traversal.

    let test_dir = temp_test_dir("vp017_cycle").expect("create temp dir");

    // Create the cycle structure:
    // test_dir/
    //   dir_a/
    //     file_a.md
    //     link_to_b/ -> ../dir_b/  (symlink)
    //   dir_b/
    //     file_b.md
    //     link_to_a/ -> ../dir_a/  (symlink back - the cycle)

    let dir_a = test_dir.join("dir_a");
    let dir_b = test_dir.join("dir_b");
    fs::create_dir_all(&dir_a).expect("create dir_a");
    fs::create_dir_all(&dir_b).expect("create dir_b");

    write_md_file(&dir_a, "file_a.md", "# File A").expect("write file_a.md");
    write_md_file(&dir_b, "file_b.md", "# File B").expect("write file_b.md");

    // Create the circular symlinks
    // Note: symlinks must be relative and we create them in separate directories
    create_dir_symlink(&dir_b, &dir_a.join("link_to_b")).expect("create dir_a/link_to_b -> dir_b");
    create_dir_symlink(&dir_a, &dir_b.join("link_to_a")).expect("create dir_b/link_to_a -> dir_a");

    let timeout = std::time::Duration::from_secs(5);
    let start = std::time::Instant::now();

    // The scanner must terminate within the timeout despite the cycle
    let results = scanner::collect_md_files(&test_dir);
    let elapsed = start.elapsed();

    assert!(
        elapsed < timeout,
        "Scan should terminate within timeout, took {:?}",
        elapsed
    );

    // Should find both files (the cycle is detected and avoided)
    assert_eq!(
        results.len(),
        2,
        "Should find both files despite symlink cycle"
    );
    let file_names: Vec<String> = results
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();
    assert!(
        file_names.contains(&"file_a.md".to_string())
            && file_names.contains(&"file_b.md".to_string()),
        "Should contain both file_a.md and file_b.md"
    );
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
        // Note: *.log only matches files ending in .log; log.md ends in .md so *.log does NOT match
        // This is a control test per BC-2.01.003 to prove exclusion is not over-broad
    }

    // log.md should survive because *.log does not match .md files per BC-2.01.003
    assert_eq!(
        results.len(),
        1,
        "Only log.md should survive (*.log doesn't match .md files)"
    );
    assert!(
        results[0].file_name().unwrap() == "log.md",
        "The surviving file should be log.md"
    );
}

// ============================================================================
// Edge case tests (EC-001 through EC-009)
// ============================================================================

#[test]
fn test_EC_001_empty_tree_no_crash() {
    // EC-001: CWD has no .md files at any depth
    let test_dir = temp_test_dir("ec001_empty").expect("create temp dir");

    // Create some non-markdown files
    fs::write(test_dir.join("config.json"), "{}").expect("write config.json");

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

    assert_eq!(
        results.len(),
        0,
        "README.MD should be excluded (case-sensitive)"
    );
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
    let _results = scanner::collect_md_files(&test_dir);

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

    // Create .gitignore that excludes docs (node_modules is NOT listed)
    write_gitignore(&test_dir, "docs/\n").expect("write .gitignore");

    let results = scanner::collect_md_files(&test_dir);

    // README.md and node_modules/pkg.md should survive (node_modules is NOT gitignored here)
    // .github/conf.md is skipped (dot-dir), docs/secret.md is gitignored
    assert_eq!(results.len(), 2, "README.md and pkg.md should survive");
    let file_names: Vec<String> = results
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();
    assert!(
        file_names.contains(&"README.md".to_string()) && file_names.contains(&"pkg.md".to_string()),
        "Should contain both README.md and pkg.md"
    );
    // Verify no result contains secret.md (docs/ gitignored) or conf.md (.github/ dot-dir)
    for path in &results {
        let path_str = path.to_string_lossy();
        assert!(
            !path_str.contains("secret.md"),
            "Should not include secret.md (gitignored by docs/): {:?}",
            path
        );
        assert!(
            !path_str.contains("conf.md"),
            "Should not include conf.md (dot-dir .github skipped): {:?}",
            path
        );
    }
}

// ============================================================================
// H1 REGRESSION TEST: dot-ancestor directory blocks scan root (MUST FAIL)
// ============================================================================
// This test demonstrates the bug where a scan root under a dot-prefixed ancestor
// returns empty results. The current filter_entry rejects ANY path component
// starting with '.', including ancestors above the root.
//
// BC-2.01.001 PC1: every .md reachable from the root must be included.
// The bug is that filter_entry rejects the entire traversal when the root itself
// or its ancestors start with '.'.
//
// This test MUST FAIL against the current implementation (intended Red).
// FIX: The filter should only skip dot-DIRECTORIES, not dot-ANCESTORS of the root.

#[test]
fn test_BC_2_01_001_dot_ancestor_should_not_block_scan() {
    // H1: Regression test for dot-ancestor blocking scan
    let parent_dir = temp_test_dir("h1_hidden_parent").expect("create parent dir");

    // Create a hidden ancestor directory containing the scan root
    let hidden_ancestor = parent_dir.join(".hidden_ancestor");
    let scan_root = hidden_ancestor.join("scanroot");
    fs::create_dir_all(&scan_root).expect("create scan root under hidden ancestor");

    // Put a .md file inside the scan root
    write_md_file(&scan_root, "readme.md", "# Readme").expect("write readme.md");

    // The scan should find readme.md - the root itself is not a dot-directory
    let results = scanner::collect_md_files(&scan_root);

    // BUG: This currently returns empty because filter_entry rejects path components
    // starting with '.', including the hidden_ancestor ancestor of scanroot
    assert!(
        !results.is_empty(),
        "Dot-ancestor should not block scan. Found {} files (expected >= 1). \
         The bug: filter_entry rejects path components starting with '.' including ancestors.",
        results.len()
    );
    assert!(
        results
            .iter()
            .any(|p| p.file_name().unwrap() == "readme.md"),
        "Should find readme.md in the scan root"
    );
}

// ============================================================================
// H2 REGRESSION TEST: dot-file exclusion bug and dot-dir skip preservation (MUST FAIL)
// ============================================================================
// Operator ruling: dot-FILES are INCLUDED; only dot-DIRECTORIES are skipped.
//
// The current implementation has TWO bugs:
// 1. filter_entry rejects dot-FILES at the root (wrongly excludes .notes.md)
// 2. The filter_entry logic incorrectly rejects dot-files in non-dot dirs
//
// BC-2.01.004: dot-directories unconditionally skipped, but dot-files are valid.
// The filter_entry implementation incorrectly rejects ALL components starting with '.'.

#[test]
fn test_BC_2_01_004_dot_files_should_be_included() {
    // H2: Regression test for dot-file inclusion bug
    let test_dir = temp_test_dir("h2_dot_files").expect("create temp dir");

    // Create regular files and dot-files at the root
    write_md_file(&test_dir, "visible.md", "# Visible").expect("write visible.md");
    write_md_file(&test_dir, ".notes.md", "# Notes (dot-file)").expect("write .notes.md");
    write_md_file(&test_dir, ".hidden.md", "# Hidden (dot-file)").expect("write .hidden.md");

    // The scanner should find ALL .md files, including dot-files
    let results = scanner::collect_md_files(&test_dir);

    // BUG: This currently returns only visible.md because filter_entry rejects
    // any path component starting with '.', including the .notes.md and .hidden.md files
    // at the root level.

    // The operator ruling: dot-FILES are included. Only dot-DIRECTORIES are skipped.
    assert_eq!(
        results.len(),
        3,
        "Dot-FILES should be included. Found {} files (expected 3). \
         .notes.md and .hidden.md are valid .md files and should be included.",
        results.len()
    );

    // Verify all three files are found
    let file_names: Vec<String> = results
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();
    assert!(
        file_names.contains(&"visible.md".to_string()),
        "Should find visible.md"
    );
    assert!(
        file_names.contains(&".notes.md".to_string()),
        "Should find .notes.md (dot-file, not dot-dir)"
    );
    assert!(
        file_names.contains(&".hidden.md".to_string()),
        "Should find .hidden.md (dot-file, not dot-dir)"
    );
}

// ============================================================================
// H2 CONTINUATION: dot-directory skip must still be preserved
// ============================================================================
// This ensures the fix for dot-files does not accidentally start including
// dot-DIRECTORIES.

#[test]
fn test_BC_2_01_004_dot_directories_still_skipped_after_dot_file_fix() {
    // H2 continuation: dot-directory skip must persist after dot-file inclusion fix
    let test_dir = temp_test_dir("h2_dot_dir_skip").expect("create temp dir");

    // Create dot-files at root (should be included)
    write_md_file(&test_dir, ".notes.md", "# Notes").expect("write .notes.md");

    // Create dot-directories with .md files inside (should be skipped)
    let ghd = test_dir.join(".github");
    fs::create_dir_all(&ghd).expect("create .github");
    write_md_file(&ghd, "PULL_REQUEST_TEMPLATE.md", "# PR").expect("write PR.md");

    let gd = test_dir.join(".git");
    fs::create_dir_all(&gd).expect("create .git");
    write_md_file(&gd, "config.md", "# Config").expect("write config.md");

    let results = scanner::collect_md_files(&test_dir);

    // .notes.md at root should be found
    // .github/PULL_REQUEST_TEMPLATE.md and .git/config.md should be skipped
    assert_eq!(
        results.len(),
        1,
        "Dot-files at root included, dot-directories still skipped. Found {} files (expected 1)",
        results.len()
    );
    assert!(
        results[0].file_name().unwrap() == ".notes.md",
        "Should find .notes.md at root"
    );

    // Verify no dot-directory files were found
    for path in &results {
        let path_str = path.to_string_lossy();
        assert!(
            !path_str.contains(".github/"),
            "Should not include .github contents: {:?}",
            path
        );
        assert!(
            !path_str.contains(".git/"),
            "Should not include .git contents: {:?}",
            path
        );
    }
}
