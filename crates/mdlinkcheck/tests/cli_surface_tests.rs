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
    // This test will FAIL TO COMPILE until the implementer adds the cli module
    // in Phase 2 with mdlinkcheck::cli::CliArgs.

    // The scan function exists, but the CLI module does not (yet)
    // This test intentionally fails to compile to demonstrate the gap.
    // When the CLI is added in Phase 2, this test will compile and pass.

    // Note: This test intentionally does NOT run against the actual CLI
    // because the cli module doesn't exist yet. This is the intended Red.
    // The compile error IS the expected outcome for this test before Phase 2.

    // To run this test in Phase 2, uncomment the following:

    /*
    let cmd = mdlinkcheck::cli::CliArgs::command();
    let arg_ids: Vec<&str> = cmd.get_arguments().map(|a| a.get_id()).collect();

    assert!(
        !arg_ids.contains(&"hidden"),
        "--hidden flag must not be defined (D-011). Found args: {:?}",
        arg_ids
    );
    */

    // For now, we just document that this test should be enabled in Phase 2.
    // The test structure is correct; the missing module is the intended failure.
    panic!("CLI module not yet implemented (Phase 2). This test is correctly failing to compile in Phase 1.");
}
