use std::path::Path;

use ignore::WalkBuilder;

/// Build a WalkBuilder configured for markdown file scanning.
///
/// The builder is configured with:
/// - .gitignore and .ignore file support
/// - Dot-directory skipping (unconditional)
/// - Directory symlink non-following
/// - Case-sensitive .md extension filtering
pub fn build_walk(root: &Path) -> WalkBuilder {
    todo!()
}

/// Collect all .md files under the given root directory.
///
/// Returns a vector of absolute paths to all markdown files
/// discovered during traversal, respecting .gitignore exclusions
/// and dot-directory skipping.
pub fn collect_md_files(root: &Path) -> Vec<PathBuf> {
    todo!()
}

/// Check if a path has the exact .md extension (case-sensitive).
///
/// Returns true only if the file ends with exactly ".md" (lowercase).
/// Returns false for .MD, .Md, .markdown, .mdx, etc.
pub fn is_md_extension(path: &Path) -> bool {
    todo!()
}
