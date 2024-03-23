

/// Converts a string to `*const i8` for use with raylib
#[macro_export]
macro_rules! rl_str {
    ($expression:expr) => {
        format!("{}\0", $expression).as_ptr() as *const i8
    };
}