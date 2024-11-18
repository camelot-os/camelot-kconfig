use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
macro_rules! println {
    () => ();
    ($($arg:tt)*) => ();
}
