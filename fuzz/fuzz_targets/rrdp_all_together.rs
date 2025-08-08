#![no_main]
use libfuzzer_sys::fuzz_target;
use rpki::rrdp::{NotificationFile, Snapshot, Delta};
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let cursor = Cursor::new(data);

    // Try to parse as NotificationFile
    let _ = NotificationFile::parse(cursor.clone());

    // Try to parse as Snapshot
    let _ = Snapshot::parse(cursor.clone());

    // Try to parse as Delta
    let _ = Delta::parse(cursor);
});
