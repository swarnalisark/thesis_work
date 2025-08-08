//! Fuzzing target for parsing Notification, Snapshot, and Delta files in RRDP.

#![no_main]

use libfuzzer_sys::fuzz_target;
use rpki::rrdp::{NotificationFile, Snapshot, Delta};
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let reader = Cursor::new(data);

    // Try parsing as Notification
    if let Ok(notification) = NotificationFile::parse(reader.clone()) {
        let _ = notification.session_id();
    }

    // Try parsing as Snapshot
    if let Ok(snapshot) = Snapshot::parse(reader.clone()) {
        let _ = snapshot.session_id();
    }

    // Try parsing as Delta
    if let Ok(delta) = Delta::parse(reader) {
        let _ = delta.session_id();
    }
});
