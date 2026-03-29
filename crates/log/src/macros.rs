#[macro_export]
macro_rules! event {
    (@tail Fatal $msg:literal) => {
        panic!($msg);
    };

    (@tail $verbosity:ident $msg:literal) => {};

    ($force:literal $verbosity:ident $msg:literal $(, $name:ident = $value:expr)*) => {
        {
            #[allow(unused_imports)]
            use $crate::prelude::{Event, Verbosity, Field, event, write, should_write};
            use $crate::value::to_value;

            if should_write(Verbosity::$verbosity) {
                write(
                    Event::new(
                        Verbosity::$verbosity,
                        module_path!(),
                        format!($msg),
                        [
                            $(
                            Field {
                                name: stringify!($name),
                                value: to_value!($value),
                            }
                            )*
                        ].into()
                    ),
                    $force
                );
            }

            event!(@tail $verbosity $msg);
        }
    };
}

#[macro_export]
macro_rules! fatal {
    (important $($t:tt)+) => { $crate::prelude::event!(true  Fatal $($t)+) };
    (ignorable $($t:tt)+) => { $crate::prelude::event!(false Fatal $($t)+) };
    ($($t:tt)+)           => { $crate::prelude::fatal!(important $($t)+) };
}

#[macro_export]
macro_rules! error {
    (important $($t:tt)+) => { $crate::prelude::event!(true  Error $($t)+) };
    (ignorable $($t:tt)+) => { $crate::prelude::event!(false Error $($t)+) };
    ($($t:tt)+)           => { $crate::prelude::error!(important $($t)+) };
}

#[macro_export]
macro_rules! warn {
    (important $($t:tt)+) => { $crate::prelude::event!(true  Warn $($t)+) };
    (ignorable $($t:tt)+) => { $crate::prelude::event!(false Warn $($t)+) };
    ($($t:tt)+)           => { $crate::prelude::warn!(ignorable $($t)+) };
}

#[macro_export]
macro_rules! info {
    (important $($t:tt)+) => { $crate::prelude::event!(true  Info $($t)+) };
    (ignorable $($t:tt)+) => { $crate::prelude::event!(false Info $($t)+) };
    ($($t:tt)+)           => { $crate::prelude::info!(ignorable $($t)+) };
}

#[macro_export]
macro_rules! debug {
    (important $($t:tt)+) => { $crate::prelude::event!(true  Debug $($t)+) };
    (ignorable $($t:tt)+) => { $crate::prelude::event!(false Debug $($t)+) };
    ($($t:tt)+)           => { $crate::prelude::debug!(ignorable $($t)+) };
}

pub use event;
pub use fatal;
pub use error;
pub use crate::warn;
pub use info;
pub use debug;
