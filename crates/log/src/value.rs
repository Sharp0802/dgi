use std::fmt::{Debug, Display};
use serde::Serialize;
use serde_json::{Map, Value};

pub struct Wrapper<'a, T>(pub &'a T);

pub trait TrySerialize {
    fn to_value(self) -> Value;
}

impl<'a, T: Serialize> TrySerialize for &&Wrapper<'a, T> {
    fn to_value(self) -> Value {
        let value = serde_json::to_value(self.0);

        if cfg!(debug_assertions) {
            value.unwrap()
        } else {
            value.unwrap_or_else(|e| Value::Object(Map::from_iter([
                ("message".into(), Value::String("serialization failed".into())),
                ("inner".into(), Value::String(e.to_string())),
            ].into_iter())))
        }
    }
}

pub trait TryDisplay {
    fn to_value(self) -> Value;
}

impl<'a, T: Display> TryDisplay for &Wrapper<'a, T> {
    fn to_value(self) -> Value {
        Value::String(self.0.to_string())
    }
}

pub trait TryDebug {
    fn to_value(self) -> Value;
}

impl<'a, T: Debug> TryDebug for Wrapper<'a, T> {
    fn to_value(self) -> Value {
        Value::String(format!("{:?}", self.0))
    }
}

#[macro_export]
macro_rules! to_value {
    ($e:expr) => {
        {
            #[allow(unused_imports)]
            use $crate::value::{Wrapper, TrySerialize, TryDisplay, TryDebug};
            (&&Wrapper(&$e)).to_value()
        }
    }
}

pub use to_value;
