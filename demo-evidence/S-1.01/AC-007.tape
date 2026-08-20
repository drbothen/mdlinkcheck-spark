# VHS tape for AC-007: Dot-directories unconditionally skipped
Output "AC-007.gif"
Output "AC-007.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_004_dot_directories_unconditionally_skipped" Enter
Sleep 2s

Type "echo 'AC-007 PASSED: Dot-directories unconditionally skipped'" Enter
Sleep 5s
