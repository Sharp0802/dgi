#[macro_export]
macro_rules! expect {
    ($e:expr, $msg:literal) => {
        match $e {
            Ok(v) => v,
            Err(e) => $crate::fatal!($msg, error = e),
        }
    }
}

pub use crate::expect;
