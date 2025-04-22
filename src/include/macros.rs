// Macros imported using `include!` directive because in a Procedural Macro Library
// it is not possible use #[macro_export] and then use macros in other modules except
// the one where they are defined.

// tracing with debug level
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug")]
        eprintln!("[PINNY][DEBUG][PID: {}] {}", std::process::id(), format_args!($($arg)*));
    };
}
