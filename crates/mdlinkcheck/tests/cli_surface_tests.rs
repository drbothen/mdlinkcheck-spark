//! CLI surface tests for S-1.01
//! Tests for AC-008 and BC-2.01.004 D-011: no --hidden override flag

use clap::CommandFactory;

// ============================================================================
// H6: test_BC_2_01_004_no_hidden_flag_defined
// ============================================================================
// D-011: The CLI must NOT define a --hidden override flag.
// This test verifies that the clap parser does not include a "hidden" argument.

#[test]
fn test_BC_2_01_004_no_hidden_flag_defined() {
    // H6: Verify the CLI surface does not define --hidden flag (D-011)
    // Positive-pinning over absence-assertion: assert all expected flags ARE present
    // AND that hidden is ABSENT

    let cmd = mdlinkcheck::cli::CliArgs::command();
    let arg_ids: Vec<String> = cmd.get_arguments().map(|a| a.get_id().as_str().to_string()).collect();

    // Must have the three domain flags: path, online, format
    assert!(
        arg_ids.contains(&"path".to_string()),
        "path flag must be present. Found args: {:?}",
        arg_ids
    );
    assert!(
        arg_ids.contains(&"online".to_string()),
        "online flag must be present. Found args: {:?}",
        arg_ids
    );
    assert!(
        arg_ids.contains(&"format".to_string()),
        "format flag must be present. Found args: {:?}",
        arg_ids
    );

    // Must NOT have hidden flag
    assert!(
        !arg_ids.contains(&"hidden".to_string()),
        "--hidden flag must NOT be defined (D-011). Found args: {:?}",
        arg_ids
    );
}
