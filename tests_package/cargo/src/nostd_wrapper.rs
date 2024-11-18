// SPDX-FileCopyrightText: 2024 Ledger SAS
//
// SPDX-License-Identifier: Apache-2.0

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
