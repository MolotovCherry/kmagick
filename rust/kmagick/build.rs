use std::env;
use winres;

fn main() {
    let is_windows = env::var("CARGO_CFG_WINDOWS").is_ok();
    if !is_windows {
        return
    }

    let res = winres::WindowsResource::new();
    res.compile().unwrap();
}
