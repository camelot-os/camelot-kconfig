// SPDX-License-Identifier: Apache-2.0
//
// Copyright 2024 Ledger SAS

#![no_std]

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
