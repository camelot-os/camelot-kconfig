// SPDX-FileCopyrightText: 2024 Ledger SAS
//
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
#[macro_use]
mod nostd_wrapper;

use kconfig;

fn dummy(a: u32) -> bool {
    let mut i = 0;

    while i < a {
        println!("hello");
        i = i + 1;
    }

    i > 100
}

fn test() {
    #[cfg(CONFIG_BOOL)]
    println!("CONFIG_BOOL okay");

    #[cfg(CONFIG_STR)]
    println!("CONFIG_STR okay");

    #[cfg(CONFIG_INT)]
    println!("CONFIG_INT okay");

    #[cfg(CONFIG_HEX)]
    println!("CONFIG_HEX okay");
    println!("{}", kconfig::get!("CONFIG_STR"));
    println!("{}", kconfig::get!("CONFIG_HEX"));
    println!("{}", kconfig::get!("CONFIG_HEX", u32));
    println!("{}", kconfig::get!("CONFIG_HEX_STR_HEX"));
    println!("{}", kconfig::get!("CONFIG_INT", u32));

    const A: u32 = kconfig::get!("CONFIG_INT", u32);
    dummy(A);
    const B: &str = kconfig::get!("CONFIG_INT");
    const C: u32 = kconfig::get!("CONFIG_HEX", u32);
    dummy(C);

    const { assert!(A == 42) }
    const { assert!(C == 0x42) }


}

fn main() {
    test()
}

#[cfg(target_os = "none")]
#[no_mangle]
fn _start() {
    main()
}
