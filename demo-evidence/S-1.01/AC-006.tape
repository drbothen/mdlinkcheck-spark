# VHS tape for AC-006: Gitignored file anchor table built as target
# NOTE: This AC has a deferred component (Pass 1.5 anchor table building)
# This demo shows only the in-scope "not in scan set" half of the AC.
Output "AC-006.gif"
Output "AC-006.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_003_gitignored_file_anchor_table_built_as_target" Enter
Sleep 2s

Type "echo 'AC-006 PARTIAL: Gitignored file excluded from scan set (anchor table deferred to Pass 1.5)'" Enter
Sleep 5s
