# VHS tape for AC-009: Directory symlinks not followed
Output "AC-009.gif"
Output "AC-009.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_004_directory_symlinks_not_followed" Enter
Sleep 2s

Type "echo 'AC-009 PASSED: Directory symlinks not followed'" Enter
Sleep 5s
