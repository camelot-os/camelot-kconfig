# SPDX-License-Identifier: Apache-2.0
#
# Copyright 2025 - H2lab software team
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

if meson.is_subproject() == false
  subdir_done()
endif

add_languages('rust', native: false, required: true)

# Set rust toolchain config entry according to kconfig
# Note:
#   configuration_file is done at top level meson.build in order to generate
#   rust-toolchain.toml at build root directory.
rust_toolchain_config = configuration_data()
rust_toolchain_config.set('channel', get_option('rust-channel'))
rust_toolchain_config.set('target', kconfig_data.get_unquoted('CONFIG_RUSTC_TARGET'))
rust_edition = '2024'
kconfig_rust_std = 'rust_std=' + rust_edition

rust_build_args = [
    '@' + fs.parent(kconfig_rustargs) / fs.name(kconfig_rustargs),
    target_rustargs,
    '-C', 'lto=true', '-C', 'relocation-model=pic', '-C', 'link-args=--emit-relocs'
]


kconfig_rlib = static_library(
    'kconfig',
    sources : files('kconfig' / 'lib.rs'),
    rust_abi: 'rust',
    rust_args: rust_build_args,
    override_options: [kconfig_rust_std],
    extra_files: uapi_modules,
    install: true,
)
kconfig_import_rlib = static_library(
    'kconfig',
    sources : files('kconfig_import' / 'lib.rs'),
    rust_abi: 'rust',
    rust_args: rust_build_args,
    override_options: [kconfig_rust_std],
    extra_files: uapi_modules,
    install: true,
)
