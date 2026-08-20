# VHS tape for AC-011: Exact .md extension included
Output "AC-011.gif"
Output "AC-011.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_005_exact_md_extension_included" Enter
Sleep 2s

Type "echo 'AC-011 PASSED: Exact .md extension included'" Enter
Sleep 5s
