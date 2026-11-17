// Integration test for SigmaOS-arch-mixed-reality-kernel
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use arch_mixed_reality_kernel as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
