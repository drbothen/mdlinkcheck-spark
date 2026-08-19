//! Mechanical pure-core I/O guard for S-1.01 (F-04/D-009/POL-11)
//!
//! This is a TEXTUAL lower-bound check (grep-class), not a semantic/type-level
//! proof. It can be evaded by aliasing/macros. It is a cheap mechanical guard,
//! not the full POL-11 contract. Reference: POL-11, D-009.
//!
//! This test:
//! 1. Positive-pinning: Scans all .rs files in core/src, asserts N>0, verifies
//!    types.rs was scanned (closed enumeration).
//! 2. Differential probe: Asserts matcher matches synthetic `use std::fs;`.
//! 3. Fails CLOSED if src dir missing/unreadable.

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

// Forbidden import/usage patterns (closed enumeration)
const FORBIDDEN_PATTERNS: &[&str] = &[
    "std::fs",
    "std::net",
    "std::io::stdout",
    "std::io::stdin",
    "std::io::stderr",
    "Instant::now",
    "rand::",
    "rand::rng",
];

#[test]
fn test_pure_core_guard_scans_files_and_detects_forbidden_patterns() {
    let src_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src"));

    // Fail CLOSED if src dir is missing or unreadable
    assert!(
        src_dir.exists(),
        "core src directory must exist: {:?}",
        src_dir
    );
    assert!(
        src_dir.is_dir(),
        "core src path must be a directory: {:?}",
        src_dir
    );

    // Read all .rs files under src/
    let entries = fs::read_dir(&src_dir).expect("failed to read src directory");
    let mut rs_files: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let entry = entry.expect("failed to read directory entry");
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "rs") {
            rs_files.push(path);
        }
    }

    // POSITIVE-PINNING: Assert we scanned at least one file
    assert!(
        !rs_files.is_empty(),
        "must scan at least one .rs file in core/src, found 0"
    );

    // Assert by closed enumeration that known files were scanned
    // At minimum: types.rs must be in the scanned set
    let scanned_names: HashSet<String> = rs_files
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    assert!(
        scanned_names.contains("types.rs"),
        "types.rs must be scanned (known core source file)"
    );

    // Print reached-count (N files scanned)
    eprintln!(
        "PURE-CORE-GUARD: Scanned {} core src files: {:?}",
        rs_files.len(),
        rs_files
    );

    // DIFFERENTIAL PROBE: Assert matcher works against synthetic string
    let synthetic_violation = "use std::fs::File;";
    let synthetic_match = check_forbidden_patterns(synthetic_violation);
    assert!(
        synthetic_match,
        "differential probe: matcher must match 'use std::fs::File;' but did not"
    );

    // Now scan actual source files
    let mut total_violations = 0;

    for file_path in &rs_files {
        let content = fs::read_to_string(file_path)
            .unwrap_or_else(|_| panic!("failed to read: {:?}", file_path));

        let violations = check_forbidden_patterns(&content);
        if violations {
            total_violations += 1;
            eprintln!(
                "PURE-CORE-GUARD VIOLATION in {:?}: forbidden patterns found",
                file_path
            );
        }
    }

    // Assert no violations (fail CLOSED if any found)
    assert_eq!(
        total_violations, 0,
        "pure-core guard: {} files contain forbidden I/O patterns",
        total_violations
    );
}

// Helper: Check if content contains any forbidden pattern
fn check_forbidden_patterns(content: &str) -> bool {
    FORBIDDEN_PATTERNS
        .iter()
        .any(|pattern| content.contains(pattern))
}
