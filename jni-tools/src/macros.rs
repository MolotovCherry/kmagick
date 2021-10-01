#[macro_export]
macro_rules! setup_panic {
    () => {
        {
            // set up empty panic handler to suppress output
            std::panic::set_hook(Box::new(|_| { }));
        }
    };
}
