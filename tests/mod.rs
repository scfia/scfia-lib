use env_logger::Target;
use log::LevelFilter;

#[macro_use]
extern crate lazy_static;

pub mod aarch64;

fn init() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter(None, LevelFilter::Trace)
        .target(Target::Stdout)
        .try_init();
}
