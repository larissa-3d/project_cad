//! Two runs of the driver at once, each in a process of its own, as two
//! checkouts testing side by side are: what one writes, the other never reads
//! nor clears.
//!
//! Closes #965.
//! - Two checkouts running `cargo test -p cao_app --test the_edge_panel --test the_alerts`
//!   at the same time both pass —
//!   `a_second_run_opening_the_same_test_leaves_what_the_first_run_saved_alone`:
//!   the timing between two checkouts is not something a test can arrange,
//!   so the collision is made to happen every time instead — a second process
//!   opens the same test while the first still holds its application, and the
//!   first reads back what it had saved
//! - The driver's tests and every app test still pass — no test: held by
//!   `cargo test -p cao_app` going green, which is a run of the suite rather
//!   than something an assertion reaches
//!
//! The second run is this same test binary started again on this one test,
//! told by an environment variable to play the other checkout. Two checkouts
//! share the machine's temporary directory and nothing else of the driver's;
//! a child process inherits that directory, so it collides exactly where
//! they did.

mod driver;

use std::process::Command;

use cao_prefs::Settings;

const PLAY_THE_SECOND_RUN: &str = "CAO_DRIVER_PLAY_THE_SECOND_RUN";

const THE_SHARED_TEST: &str = "a_second_run_opening_the_same_test";

#[test]
fn a_second_run_opening_the_same_test_leaves_what_the_first_run_saved_alone() {
    if std::env::var_os(PLAY_THE_SECOND_RUN).is_some() {
        let _app = driver::open(THE_SHARED_TEST);
        return;
    }

    let mine = Settings {
        reopen_at_last_size: false,
        ..Settings::default()
    };
    let _app = driver::open_held_with(THE_SHARED_TEST, mine);
    assert!(
        !driver::saved_settings(THE_SHARED_TEST).reopen_at_last_size,
        "the first run reads back the settings it opened with, before any second run",
    );

    let second = Command::new(std::env::current_exe().expect("the test binary knows its path"))
        .args([
            "--exact",
            "a_second_run_opening_the_same_test_leaves_what_the_first_run_saved_alone",
            "--test-threads=1",
        ])
        .env(PLAY_THE_SECOND_RUN, "1")
        .output()
        .expect("the second run starts");
    assert!(
        second.status.success(),
        "the second run failed to open its application:\n{}\n{}",
        String::from_utf8_lossy(&second.stdout),
        String::from_utf8_lossy(&second.stderr),
    );

    assert!(
        !driver::saved_settings(THE_SHARED_TEST).reopen_at_last_size,
        "another process opening the same test wrote over what this run had saved: the two \
         runs share one place",
    );
}
