use outpost_kconfig;

fn main() {

    #[cfg(CONFIG_BOOL)]
    println!("CONFIG_BOOL okay");

    #[cfg(CONFIG_STR = "test str")]
    println!("CONFIG_STR okay");

    #[cfg(CONFIG_INT = "42")]
    println!("CONFIG_INT okay");

    #[cfg(CONFIG_HEX = "0x42")]
    println!("CONFIG_HEX okay");
    println!("{}", outpost_kconfig::get!("CONFIG_STR"));
    println!("{}", kconfig_get!("CONFIG_HEX"));
    println!("{}", kconfig_get!("CONFIG_HEX", u32));
    println!("{}", kconfig_get!("CONFIG_INT", u32));

}
