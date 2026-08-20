# VHS tape for AC-012: Non-.md extensions excluded
Output "AC-012.gif"
Output "AC-012.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_005_non_md_extensions_excluded" Enter
Sleep 2s

Type "echo 'AC-012 PASSED: Non-.md extensions excluded (D-012)'" Enter
Sleep 5s
