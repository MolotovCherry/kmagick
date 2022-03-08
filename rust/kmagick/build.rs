#[cfg(windows)]
use winres;
use winres::VersionInfo;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
