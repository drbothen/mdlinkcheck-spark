# VHS tape for AC-013: Case-sensitive byte match for .md extension
Output "AC-013.gif"
Output "AC-013.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_005_case_sensitive_byte_match" Enter
Sleep 2s

Type "echo 'AC-013 PASSED: Case-sensitive byte match for .md extension'" Enter
Sleep 5s
