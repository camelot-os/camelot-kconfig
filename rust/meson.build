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

with_rlib_opt = get_option('with_rlib')

if not with_rlib_opt
    subdir_done()
endif

# c cross-compiler needed by rust build
add_languages('c', native: false, required: true)
add_languages('rust', native: false, required: true)

target_cpu = 'cortex-@0@'.format(kconfig_data.get_unquoted('CONFIG_ARCH_ARM_CORTEX_NAME'))
c_compiler = meson.get_compiler('c')

target_rust = kconfig_data.get_unquoted('CONFIG_RUSTC_TARGET')
target_rustargs = [
    '-Ctarget-cpu='+ target_cpu,
]


# Set rust toolchain config entry according to kconfig
# Note:
#   configuration_file is done at top level meson.build in order to generate
#   rust-toolchain.toml at build root directory.
rust_edition = '2024'
kconfig_rust_std = 'rust_std=' + rust_edition

rust_build_args = [
    '@' + fs.parent(kconfig_rustargs) / fs.name(kconfig_rustargs),
    target_rustargs,
    '-C', 'lto=true', '-C', 'relocation-model=pic', '-C', 'link-args=--emit-relocs'
]


kconfig_rlib = static_library(
    'kconfig',
    sources : files('kconfig' / 'src' / 'lib.rs'),
    rust_abi: 'rust',
    rust_args: rust_build_args,
    override_options: [kconfig_rust_std],
    install: true,
)

kconfig_rust_dep = declare_dependency(link_whole : kconfig_rlib)
