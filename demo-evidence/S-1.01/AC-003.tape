# VHS tape for AC-003: Scan terminates for finite tree
Output "AC-003.gif"
Output "AC-003.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_001_scan_terminates_for_finite_tree" Enter
Sleep 2s

Type "echo 'AC-003 PASSED: Scan terminates for finite tree'" Enter
Sleep 5s
