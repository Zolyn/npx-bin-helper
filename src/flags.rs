use std::ffi::OsString;

xflags::xflags! {
    cmd app {
        optional -s, --shell type: OsString
    }
}
