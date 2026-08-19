use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use ignore::WalkBuilder;

/// Build a WalkBuilder configured for markdown file scanning.
///
/// The builder is configured with:
/// - Native .gitignore/.ignore file handling (git_ignore(true), ignore(true))
/// - require_git(false) to enable gitignore in non-git directories (e.g. temp dirs)
/// - Dot-directory skipping via filter_entry (unconditional)
/// - Directory symlink non-following (follow_links(false))
/// - Case-sensitive .md extension filtering
pub fn build_walk(root: &Path) -> WalkBuilder {
    let mut builder = WalkBuilder::new(root);
    builder
        // Native gitignore handling: respect .gitignore and .ignore files
        .git_ignore(true)
        .ignore(true)
        // Enable gitignore in directories that are not actual git repos
        .require_git(false)
        // Do not follow directory symlinks to prevent infinite loops
        .follow_links(false)
        // Set no max depth for unlimited traversal
        .max_depth(None)
        // Skip dot-directories unconditionally (dot-files are still scanned)
        .filter_entry(|entry| {
            // Skip if any path component is a dot-directory
            !entry
                .path()
                .components()
                .any(|c| matches!(c, std::path::Component::Normal(os) if os.to_string_lossy().starts_with('.')))
        });
    builder
}

/// Collect all .md files under the given root directory.
///
/// Returns a vector of absolute paths to all markdown files
/// discovered during traversal, respecting .gitignore exclusions
/// and dot-directory skipping.
pub fn collect_md_files(root: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();

    for entry in build_walk(root).build() {
        match entry {
            Ok(entry) => {
                let path = entry.path().to_path_buf();

                // Only include regular files (not directories, not symlinks to directories)
                // Symlinks to files ARE followed per the spec (BC-2.01.004 postcondition 3)
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    // Check if it has .md extension (case-sensitive)
                    if is_md_extension(&path) {
                        // Deduplicate by absolute path
                        let abs_path = path
                            .canonicalize()
                            .unwrap_or_else(|_| path.to_path_buf());
                        if seen.insert(abs_path.clone()) {
                            paths.push(abs_path);
                        }
                    }
                }
            }
            Err(_) => {
                // Skip entries we can't read
                continue;
            }
        }
    }

    paths
}

/// Check if a path has the exact .md extension (case-sensitive).
///
/// Returns true only if the file ends with exactly ".md" (lowercase).
/// Returns false for .MD, .Md, .markdown, .mdx, etc.
pub fn is_md_extension(path: &Path) -> bool {
    path.extension()
        .map(|ext| ext == "md")
        .unwrap_or(false)
}
