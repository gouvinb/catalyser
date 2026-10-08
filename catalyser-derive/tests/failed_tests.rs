//! Compile-fail tests: they pin the diagnostics users see when they misuse
//! `#[overloads]` or the generated call macro.
//!
//! The `.stderr` snapshots depend on the rustc version. To (re)generate them:
//! `TRYBUILD=overwrite cargo test -p catalyser-derive --test bad_sources`

#[test]
fn failed_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/bad_sources/*.rs");
}
