// Integration test for SigmaOS-hdr-display-support
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use hdr_display_support as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
