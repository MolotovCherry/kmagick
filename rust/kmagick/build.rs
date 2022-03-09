use std::env;
use winres;

fn main() {
    let is_windows = env::var("CARGO_CFG_WINDOWS").is_ok();
    // HOST must also be windows to run stamping tool
    if !is_windows || !cfg!(windows) {
        return
    }

    let res = winres::WindowsResource::new();
    res.compile().unwrap();
}
