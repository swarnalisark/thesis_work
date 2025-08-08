#![no_main]
use libfuzzer_sys::fuzz_target;
use rpki::rrdp::NotificationFile;   // ← corrected: no `notification::`
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    if let Ok(parsed) = NotificationFile::parse_limited(Cursor::new(data), 100) {
        // Touch a field so the value isn’t optimised away
        let _ = parsed.session_id();
    }
});
