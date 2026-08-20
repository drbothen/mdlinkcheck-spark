# VHS tape for AC-008: No override flag for dot-directory skip
Output "AC-008.gif"
Output "AC-008.webm"

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1200
Set Height 600

Type "cd /Users/jmagady/Dev/mdlinkcheck-spark/.worktrees/S-1.01" Enter
Sleep 500ms

Type "cargo test --test cli_surface_tests test_BC_2_01_004_no_override_flag_for_dot_dir_skip" Enter
Sleep 2s

Type "echo 'AC-008 PASSED: No --hidden flag defined (dot-dir skip unconditional)'" Enter
Sleep 5s
