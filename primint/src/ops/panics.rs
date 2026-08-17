#[cold]
#[track_caller]
pub fn ilog_negative() -> ! {
    panic!("Argument to ilog must be positive")
}
