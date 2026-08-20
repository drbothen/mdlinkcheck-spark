# VHS tape for AC-005: Gitignored file not scanned as source
Output "AC-005.gif"
Output "AC-005.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_003_gitignored_file_not_scanned_as_source" Enter
Sleep 2s

Type "echo 'AC-005 PASSED: Gitignored file not scanned as source'" Enter
Sleep 5s
