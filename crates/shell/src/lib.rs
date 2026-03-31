#![feature(sync_nonpoison)]
#![feature(nonpoison_rwlock)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod app;
mod resource;
mod util;
