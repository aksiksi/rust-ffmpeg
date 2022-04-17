use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        const CORRUPT = AV_FRAME_FLAG_CORRUPT;
        const DISCARD = AV_FRAME_FLAG_DISCARD;
    }
}
