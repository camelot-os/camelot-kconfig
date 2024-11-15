// SPDX-License-Identifier: Apache-2.0
//
// Copyright 2024 Ledger SAS

use std::collections::HashMap;
use std::env;

pub struct DotConfig<'a>(HashMap<&'a str, &'a str>);

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

impl<'a> IntoIterator for DotConfig<'a> {
    type Item = <HashMap<&'a str, &'a str> as IntoIterator>::Item;
    type IntoIter = <HashMap<&'a str, &'a str> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn add_cfg<'a>(name: &'a str) {
    println!("cargo::rustc-check-cfg=cfg({name},values(none()))");
    println!("cargo::rustc-cfg={name}");
}

fn add_option_env<'a>(name: &'a str, value: &'a str) {
    match value.strip_prefix("0x") {
        Some(stripped) => {
            let numerical_value = u64::from_str_radix(stripped, 16).unwrap();
            println!("cargo::rustc-env={name}={numerical_value}");
            println!("cargo::rustc-env={name}_STR_HEX={value}");
        },
        None => println!("cargo::rustc-env={name}={value}"),
    }
}

pub fn import_dotconfig_from_script() {
    let dotconfig_filename = &env::var("config").unwrap_or(".config".to_string());
    let data = std::fs::read_to_string(dotconfig_filename).unwrap();
    let dotconfig = DotConfig::from(data.as_str());

    println!("cargo::rerun-if-env-changed=config");
    println!("cargo::rerun-if-changed={dotconfig_filename}");

    for (key, value) in dotconfig.into_iter() {
        match value {
            "n" | "m"   => (), // skip boolean (or tristate) set to n (or m)
            "y"         => add_cfg(key),
            _           => {
                add_cfg(key);
                add_option_env(key, value)
            },
        }
    }
}

#[macro_export]
macro_rules! _get {
    ($value:ident) => {$value};
    ($value:ident, $value_type:ty) => {
        match <$value_type>::from_str_radix($value, 10) {
            Ok(v) => v,
            Err(e) => panic!("kconfig conversion error"),
        }
    };
}

#[macro_export]
macro_rules! get {
    ($name:expr $(, $value_type:ty)?) => {{
        const value: &str = match option_env!($name) {
            Some(v) => v,
            None => panic!("kconfig entry not found !"),
        };

        use $crate::_get;
        _get!(value $(, $value_type)?)
    }};
}
