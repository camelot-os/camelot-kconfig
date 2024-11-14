// SPDX-License-Identifier: Apache-2.0
//
// Copyright 2024 Ledger SAS

use std::collections::HashMap;
use std::env;

pub struct DotConfig<'a>(pub HashMap<&'a str, &'a str>);

impl<'a> From<HashMap<&'a str, &'a str>> for DotConfig<'a> {
    fn from(map: HashMap<&'a str, &'a str>) -> Self {
        Self(map)
    }
}

impl<'a> From<&'a str> for DotConfig<'a> {
    fn from(s: &'a str) -> Self {
        let mut map = HashMap::new();
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line.split_once('=').unwrap();
            eprintln!("{key} = {value}");
            map.insert(key.trim(), value.trim().trim_matches('"'));
        }
        DotConfig::from(map)
    }
}

pub fn import_dotconfig_from_script() {
    let dotconfig_filename = &env::var("config").unwrap_or(".config".to_string());
    let data = std::fs::read_to_string(dotconfig_filename).unwrap();
    let dotconfig = DotConfig::from(data.as_str());

    println!("cargo::rerun-if-env-changed=config");
    println!("cargo::rerun-if-changed={dotconfig_filename}");

    // XXX:
    // dotconfig does not extend HashMap but it's a tuple w/ one member
    // Thus, one needs to access first tuple element
    // TODO: Is it possible to extend type (or at least, implements iter trait) ?
    for (key, value) in dotconfig.0.into_iter() {
        if value == "y" {
            println!("cargo::rustc-check-cfg=cfg({key},values(none()))");
            println!("cargo::rustc-cfg={key}");
        } else {
            println!("cargo::rustc-check-cfg=cfg({key},values(\"{value}\"))");
            println!("cargo::rustc-cfg={key}=\"{value}\"");
            println!("cargo::rustc-env={key}={value}");
        }
    }
}

#[macro_export]
macro_rules! _get {
    ($value:ident) => {$value};
    ($value:ident, $value_type:ty) => {
        if $value.starts_with("0x") {
            <$value_type>::from_str_radix($value.trim_start_matches(&"0x"), 16).unwrap()
        } else {
            <$value_type>::from_str_radix($value, 10).unwrap()
        }
    };
}

#[macro_export]
macro_rules! get {
    ($name:expr $(, $value_type:ty)?) => {{
        const value_str: Option<&str> = option_env!($name);
        const { assert!(value_str.is_some()) }
        let value = value_str.unwrap();

        use $crate::_get;
        _get!(value $(, $value_type)?)
    }};
}
