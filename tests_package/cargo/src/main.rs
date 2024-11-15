use outpost_kconfig;

fn main() {

    #[cfg(CONFIG_BOOL)]
    println!("CONFIG_BOOL okay");

    #[cfg(CONFIG_STR)]
    println!("CONFIG_STR okay");

    #[cfg(CONFIG_INT)]
    println!("CONFIG_INT okay");

    #[cfg(CONFIG_HEX)]
    println!("CONFIG_HEX okay");
    println!("{}", outpost_kconfig::get!("CONFIG_STR"));
    println!("{}", outpost_kconfig::get!("CONFIG_HEX"));
    println!("{}", outpost_kconfig::get!("CONFIG_HEX", u32));
    println!("{}", outpost_kconfig::get!("CONFIG_HEX_STR_HEX"));
    println!("{}", outpost_kconfig::get!("CONFIG_INT", u32));

    const a: u32 = outpost_kconfig::get!("CONFIG_INT", u32);
    const { assert!(a == 42) }

}
