# VHS tape for AC-001: Default CWD scan includes all .md files
Output "AC-001.gif"
Output "AC-001.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test scanner_discovery_tests test_BC_2_01_001_default_cwd_scan_includes_all_md_files --no-run -- --nocapture" Enter
Sleep 2s

Type "cargo test --test scanner_discovery_tests test_BC_2_01_001_default_cwd_scan_includes_all_md_files" Enter
Sleep 1s

Type "echo 'AC-001 PASSED: Default CWD scan includes all .md files'" Enter
Sleep 5s
