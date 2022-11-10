use std::ffi::OsString;

xflags::xflags! {
    /// Generate commands that add node_modules/.bin to PATH
    cmd npx-bin-helper {
        default cmd env {}
        /// Generate shell setup script
        cmd setup {}
        /// Shell type
        required -s, --shell type: OsString
    }
}
