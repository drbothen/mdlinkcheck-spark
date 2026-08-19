//! Unit tests for S-1.01: Shared Types in mdlinkcheck-core
//!
//! Tests verify correct construction of:
//! - Link
//! - ExtractedLink
//! - Finding
//! - Verdict
//! - AnchorTable
//! - DirIndex
//! - DirEntryInfo
//! - EntryKind

use mdlinkcheck_core::types::*;
use std::path::PathBuf;

// ============================================================================
// Unit Tests for Link Type
// ============================================================================

#[test]
fn test_link_construction() {
    let link = Link {
        source: PathBuf::from("/path/to/source.md"),
        raw_dest: "./target.md".to_string(),
        dest: "target.md".to_string(),
        line: 42,
        col: 15,
    };

    assert_eq!(link.source.to_string_lossy(), "/path/to/source.md");
    assert_eq!(link.raw_dest, "./target.md");
    assert_eq!(link.dest, "target.md");
    assert_eq!(link.line, 42);
    assert_eq!(link.col, 15);
}

#[test]
fn test_link_equality() {
    let link1 = Link {
        source: PathBuf::from("/a.md"),
        raw_dest: "b.md".to_string(),
        dest: "b.md".to_string(),
        line: 1,
        col: 1,
    };

    let link2 = Link {
        source: PathBuf::from("/a.md"),
        raw_dest: "b.md".to_string(),
        dest: "b.md".to_string(),
        line: 1,
        col: 1,
    };

    let link3 = Link {
        source: PathBuf::from("/a.md"),
        raw_dest: "c.md".to_string(),
        dest: "c.md".to_string(),
        line: 1,
        col: 1,
    };

    assert_eq!(link1, link2);
    assert_ne!(link1, link3);
}

#[test]
fn test_link_hashing() {
    let link = Link {
        source: PathBuf::from("/test.md"),
        raw_dest: "target.md".to_string(),
        dest: "target.md".to_string(),
        line: 1,
        col: 1,
    };

    // Test that Link can be used in a HashSet
    let mut set = std::collections::HashSet::new();
    set.insert(link.clone());

    assert!(set.contains(&link));
}

#[test]
fn test_link_serialization() {
    let link = Link {
        source: PathBuf::from("/source.md"),
        raw_dest: "target.md".to_string(),
        dest: "target.md".to_string(),
        line: 10,
        col: 5,
    };

    let json = serde_json::to_string(&link).expect("serialize Link");
    // Note: types only implement Serialize, not Deserialize
    // This test verifies serialization works; deserialization would require
    // the type to implement both Serialize and Deserialize

    // Verify the JSON contains expected fields
    assert!(json.contains("source"));
    assert!(json.contains("raw_dest"));
    assert!(json.contains("dest"));
    assert!(json.contains("line"));
    assert!(json.contains("col"));
}

// ============================================================================
// Unit Tests for ExtractedLink Type
// ============================================================================

#[test]
fn test_extracted_link_construction() {
    let link = ExtractedLink {
        raw_dest: "./target.md".to_string(),
        dest: "target.md".to_string(),
        line: 25,
        col: 8,
    };

    assert_eq!(link.raw_dest, "./target.md");
    assert_eq!(link.dest, "target.md");
    assert_eq!(link.line, 25);
    assert_eq!(link.col, 8);
}

#[test]
fn test_extracted_link_equality() {
    let link1 = ExtractedLink {
        raw_dest: "target.md".to_string(),
        dest: "target.md".to_string(),
        line: 1,
        col: 1,
    };

    let link2 = ExtractedLink {
        raw_dest: "target.md".to_string(),
        dest: "target.md".to_string(),
        line: 1,
        col: 1,
    };

    assert_eq!(link1, link2);
}

#[test]
fn test_extracted_link_serialization() {
    let link = ExtractedLink {
        raw_dest: "./target.md".to_string(),
        dest: "target.md".to_string(),
        line: 100,
        col: 20,
    };

    let json = serde_json::to_string(&link).expect("serialize ExtractedLink");

    // Verify the JSON contains expected fields
    assert!(json.contains("raw_dest"));
    assert!(json.contains("dest"));
    assert!(json.contains("line"));
    assert!(json.contains("col"));
}

// ============================================================================
// Unit Tests for Finding Type
// ============================================================================

#[test]
fn test_finding_construction() {
    let verdict = Verdict::FileNotFound;
    let finding = Finding {
        path: PathBuf::from("/source.md"),
        link_target: "missing.md".to_string(),
        line: 15,
        col: 3,
        verdict,
        reason: Some("Target file does not exist".to_string()),
    };

    assert_eq!(finding.path.to_string_lossy(), "/source.md");
    assert_eq!(finding.link_target, "missing.md");
    assert_eq!(finding.line, 15);
    assert_eq!(finding.col, 3);
    assert_eq!(finding.verdict, Verdict::FileNotFound);
    assert_eq!(
        finding.reason,
        Some("Target file does not exist".to_string())
    );
}

#[test]
fn test_finding_without_reason() {
    let finding = Finding {
        path: PathBuf::from("/source.md"),
        link_target: "broken.md".to_string(),
        line: 5,
        col: 10,
        verdict: Verdict::HttpError,
        reason: None,
    };

    assert!(finding.reason.is_none());
}

#[test]
fn test_finding_serialization() {
    let finding = Finding {
        path: PathBuf::from("/test/source.md"),
        link_target: "missing.md".to_string(),
        line: 42,
        col: 7,
        verdict: Verdict::AnchorNotFound,
        reason: Some("Anchor 'section' not found".to_string()),
    };

    let json = serde_json::to_string(&finding).expect("serialize Finding");

    // Verify the JSON contains expected fields
    assert!(json.contains("path"));
    assert!(json.contains("link_target"));
    assert!(json.contains("verdict"));
    assert!(json.contains("reason"));
}

// ============================================================================
// Unit Tests for Verdict Type
// ============================================================================

#[test]
fn test_verdict_variants() {
    assert_eq!(Verdict::Ok, Verdict::Ok);
    assert_ne!(Verdict::Ok, Verdict::FileNotFound);
    assert_ne!(Verdict::FileNotFound, Verdict::AnchorNotFound);
    assert_ne!(Verdict::HttpError, Verdict::MalformedUrl);
    assert_ne!(Verdict::IoError, Verdict::ConfigError);
}

#[test]
fn test_verdict_serialization() {
    let variants = vec![
        Verdict::Ok,
        Verdict::FileNotFound,
        Verdict::AnchorNotFound,
        Verdict::HttpError,
        Verdict::MalformedUrl,
        Verdict::IoError,
        Verdict::ConfigError,
    ];

    for variant in variants {
        let json = serde_json::to_string(&variant).expect("serialize Verdict");
        // Verify the JSON contains the variant name
        let variant_name = match variant {
            Verdict::Ok => "Ok",
            Verdict::FileNotFound => "FileNotFound",
            Verdict::AnchorNotFound => "AnchorNotFound",
            Verdict::HttpError => "HttpError",
            Verdict::MalformedUrl => "MalformedUrl",
            Verdict::IoError => "IoError",
            Verdict::ConfigError => "ConfigError",
        };
        assert!(json.contains(variant_name));
    }
}

#[test]
fn test_verdict_copy_semantics() {
    let v1 = Verdict::FileNotFound;
    let v2 = v1; // Copy

    assert_eq!(v1, v2);
}

// ============================================================================
// Unit Tests for AnchorTable Type
// ============================================================================

#[test]
fn test_anchor_table_construction() {
    let mut table = AnchorTable::default();
    table.anchors.insert("section1".to_string(), 10);
    table.anchors.insert("section2".to_string(), 25);

    assert_eq!(table.anchors.len(), 2);
    assert_eq!(table.anchors.get("section1"), Some(&10));
    assert_eq!(table.anchors.get("section2"), Some(&25));
}

#[test]
fn test_anchor_table_equality() {
    let mut t1 = AnchorTable::default();
    t1.anchors.insert("a".to_string(), 1);

    let mut t2 = AnchorTable::default();
    t2.anchors.insert("a".to_string(), 1);

    let mut t3 = AnchorTable::default();
    t3.anchors.insert("b".to_string(), 2);

    assert_eq!(t1, t2);
    assert_ne!(t1, t3);
}

#[test]
fn test_anchor_table_serialization() {
    let mut table = AnchorTable::default();
    table.anchors.insert("heading".to_string(), 5);
    table.anchors.insert("chapter".to_string(), 100);

    let json = serde_json::to_string(&table).expect("serialize AnchorTable");

    // Verify the JSON contains expected fields
    assert!(json.contains("anchors"));
}

// ============================================================================
// Unit Tests for DirIndex Type
// ============================================================================

#[test]
fn test_dir_index_construction() {
    let mut index = DirIndex::default();
    let dir_path = PathBuf::from("/some/dir");

    let entry = DirEntryInfo {
        name: "file.md".to_string(),
        kind: EntryKind::File,
    };

    index
        .directories
        .entry(dir_path.clone())
        .or_insert_with(Vec::new)
        .push(entry);

    assert!(index.directories.contains_key(&dir_path));
}

#[test]
fn test_dir_index_equality() {
    let mut i1 = DirIndex::default();
    let mut i2 = DirIndex::default();

    let path = PathBuf::from("/test");

    i1.directories
        .entry(path.clone())
        .or_insert_with(Vec::new)
        .push(DirEntryInfo {
            name: "a.md".to_string(),
            kind: EntryKind::File,
        });

    i2.directories
        .entry(path)
        .or_insert_with(Vec::new)
        .push(DirEntryInfo {
            name: "a.md".to_string(),
            kind: EntryKind::File,
        });

    assert_eq!(i1, i2);
}

// ============================================================================
// Unit Tests for DirEntryInfo Type
// ============================================================================

#[test]
fn test_dir_entry_info_construction() {
    let entry = DirEntryInfo {
        name: "file.md".to_string(),
        kind: EntryKind::File,
    };

    assert_eq!(entry.name, "file.md");
    assert_eq!(entry.kind, EntryKind::File);
}

#[test]
fn test_dir_entry_info_serialization() {
    let entry = DirEntryInfo {
        name: "document.md".to_string(),
        kind: EntryKind::SymlinkDir,
    };

    let json = serde_json::to_string(&entry).expect("serialize DirEntryInfo");

    // Verify the JSON contains expected fields
    assert!(json.contains("name"));
    assert!(json.contains("kind"));
}

// ============================================================================
// Unit Tests for EntryKind Type
// ============================================================================

#[test]
fn test_entry_kind_variants() {
    assert_eq!(EntryKind::File, EntryKind::File);
    assert_ne!(EntryKind::File, EntryKind::Dir);
    assert_ne!(EntryKind::File, EntryKind::SymlinkFile);
    assert_ne!(EntryKind::File, EntryKind::SymlinkDir);
    assert_ne!(EntryKind::File, EntryKind::Dangling);
}

#[test]
fn test_entry_kind_serialization() {
    let variants = vec![
        EntryKind::File,
        EntryKind::Dir,
        EntryKind::SymlinkFile,
        EntryKind::SymlinkDir,
        EntryKind::Dangling,
    ];

    for variant in variants {
        let json = serde_json::to_string(&variant).expect("serialize EntryKind");
        // Verify the JSON contains the variant name
        let variant_name = match variant {
            EntryKind::File => "File",
            EntryKind::Dir => "Dir",
            EntryKind::SymlinkFile => "SymlinkFile",
            EntryKind::SymlinkDir => "SymlinkDir",
            EntryKind::Dangling => "Dangling",
        };
        assert!(json.contains(variant_name));
    }
}

// ============================================================================
// Integration Tests: Type Combinations
// ============================================================================

#[test]
fn test_link_to_finding_conversion() {
    let link = Link {
        source: PathBuf::from("/source.md"),
        raw_dest: "target.md".to_string(),
        dest: "target.md".to_string(),
        line: 5,
        col: 10,
    };

    // Create a Finding from a Link
    let finding = Finding {
        path: link.source.clone(),
        link_target: link.dest.clone(),
        line: link.line,
        col: link.col,
        verdict: Verdict::FileNotFound,
        reason: None,
    };

    assert_eq!(finding.path, link.source);
    assert_eq!(finding.link_target, link.dest);
}

#[test]
fn test_complex_anchor_table_with_multiple_entries() {
    let mut table = AnchorTable::default();

    for i in 1..=50 {
        table.anchors.insert(format!("heading_{}", i), i * 10);
    }

    assert_eq!(table.anchors.len(), 50);
    assert_eq!(table.anchors.get("heading_1"), Some(&10));
    assert_eq!(table.anchors.get("heading_50"), Some(&500));
}

#[test]
fn test_dir_index_with_mixed_entry_kinds() {
    let mut index = DirIndex::default();
    let path = PathBuf::from("/root");

    let entries = vec![
        DirEntryInfo {
            name: "file.md".to_string(),
            kind: EntryKind::File,
        },
        DirEntryInfo {
            name: "subdir".to_string(),
            kind: EntryKind::Dir,
        },
        DirEntryInfo {
            name: "link_to_file".to_string(),
            kind: EntryKind::SymlinkFile,
        },
        DirEntryInfo {
            name: "link_to_dir".to_string(),
            kind: EntryKind::SymlinkDir,
        },
        DirEntryInfo {
            name: "broken_link".to_string(),
            kind: EntryKind::Dangling,
        },
    ];

    index
        .directories
        .entry(path.clone())
        .or_insert_with(Vec::new)
        .extend(entries);

    assert_eq!(index.directories[&path].len(), 5);
}
