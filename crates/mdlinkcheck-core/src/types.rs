use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a discovered link from a markdown file.
/// This is the core link data structure used throughout the system.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Link {
    /// The source file path where the link was found
    pub source: PathBuf,
    /// The raw link text as it appeared in the markdown
    pub raw_dest: String,
    /// The parsed destination URL/path
    pub dest: String,
    /// The line number where the link appears (1-indexed)
    pub line: usize,
    /// The column number where the link starts (1-indexed)
    pub col: usize,
}

/// Represents a link extracted from a markdown file's events.
/// This is the intermediate representation before full resolution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct ExtractedLink {
    /// The raw destination as written in markdown
    pub raw_dest: String,
    /// The destination after processing (e.g., percent-decoded)
    pub dest: String,
    /// The line number where the link appears
    pub line: usize,
    /// The column number where the link starts
    pub col: usize,
}

/// Represents a finding (issue) discovered during link checking.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    /// The source file path
    pub path: PathBuf,
    /// The link destination that failed
    pub link_target: String,
    /// The line number of the link
    pub line: usize,
    /// The column number of the link
    pub col: usize,
    /// The type of verdict (broken, ok, etc.)
    pub verdict: Verdict,
    /// Optional human-readable reason
    pub reason: Option<String>,
}

/// Represents the verdict for a single link check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Verdict {
    /// Link is valid and reachable
    Ok,
    /// Link target file does not exist
    FileNotFound,
    /// Link target anchor does not exist in the target file
    AnchorNotFound,
    /// HTTP request failed (for online mode)
    HttpError,
    /// URL syntax is invalid
    MalformedUrl,
    /// IO error occurred while checking the link
    IoError,
    /// Config error (e.g., invalid allow/deny patterns)
    ConfigError,
}

/// Represents a table of anchors found in a markdown file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct AnchorTable {
    /// Map of anchor name to its position in the file
    pub anchors: HashMap<String, usize>,
}

/// Represents the index of all directories and their entries.
/// Used for link target resolution in Pass 1.5.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DirIndex {
    /// Map from directory path to its entries
    pub directories: HashMap<PathBuf, Vec<DirEntryInfo>>,
}

/// Represents information about a directory entry.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct DirEntryInfo {
    /// The entry name (filename or directory name)
    pub name: String,
    /// The kind of entry
    pub kind: EntryKind,
}

/// Represents the kind of directory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum EntryKind {
    /// A regular file
    File,
    /// A directory
    Dir,
    /// A symlink to a file
    SymlinkFile,
    /// A symlink to a directory
    SymlinkDir,
    /// A dangling symlink (target does not exist)
    Dangling,
}
