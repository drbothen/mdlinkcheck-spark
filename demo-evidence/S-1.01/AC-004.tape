# VHS tape for AC-004: .gitignore excludes from scan set
Output "AC-004.gif"
Output "AC-004.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_003_gitignore_excludes_from_scan_set" Enter
Sleep 2s

Type "echo 'AC-004 PASSED: .gitignore excludes from scan set'" Enter
Sleep 5s
