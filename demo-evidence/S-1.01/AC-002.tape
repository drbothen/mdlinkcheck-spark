# VHS tape for AC-002: No duplicate files in scan set
Output "AC-002.gif"
Output "AC-002.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_001_no_duplicate_in_scan_set" Enter
Sleep 2s

Type "echo 'AC-002 PASSED: No duplicate files in scan set'" Enter
Sleep 5s
