use std::ffi::OsString;

xflags::xflags! {
    cmd app {
        default cmd env {}
        cmd setup {}
        required -s, --shell type: OsString
    }
}
