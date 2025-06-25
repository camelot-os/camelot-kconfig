# SPDX-License-Identifier: Apache-2.0
#
# Copyright 2023 - 2024 Ledger SAS
# Copyright 2025 H2Lab
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

project(
    'kconfig',
    meson_version: '>=1.4.0',
    version : run_command('support/meson/version.sh', 'get-vcs', check: true).stdout().strip(),
    license: 'Apache-2.0',
    license_files: ['LICENSES/Apache-2.0.txt'],
)

meson.add_dist_script('support/meson/version.sh', 'set-dist', meson.project_version())

pymod = import('python')
keyval = import('keyval')
fs = import('fs')

py3 = pymod.find_installation('python3', modules: ['kconfiglib'])

menuconfig = find_program('menuconfig')
oldconfig = find_program('oldconfig')
alldefconfig = find_program('alldefconfig')
savedefconfig = find_program('savedefconfig')

jinja_cli = find_program('jinja')

stamp_configure = find_program('scripts/stamp_configure.py')
kconfig_py = find_program('scripts/kconfig.py')

kconfig = get_option('kconfig')
kconfig_config_in = get_option('config')
kconfig_olddotconfig = meson.current_build_dir() / '.config.old'
kconfig_dotconfig = meson.current_build_dir() / '.config'
kconfig_savedefconfig =  meson.current_build_dir() / 'defconfig'

# define kconfig usefull envvars
#
# $(KCONFIG_CONFIG) is the alternate output filename for .config
# $(srctree) is the kconfig standard variable for top-level Kconfig basename
# $(subprojects) is a variable for top-level subprojects directory
#
# usage:
#  - One can use one of those to include sub-level Kconfig definition, e.g. an
# application that want to include a library Kconfig menu(entry).
#   > source $(subprojects)/<deps>/Kconfig

kconfig_srctree =  meson.global_source_root()
kconfig_subprojects_tree =  kconfig_srctree / 'subprojects'
# get back sdk root (excluding PREFIX), based on pkg_config_path
pkgconfig_paths_opt = meson.get_external_property('pkg_config_libdir', '')
if pkgconfig_paths_opt.contains('pkgconfig')
  kconfig_sdk_root_tree = fs.parent(fs.parent(pkgconfig_paths_opt))
  message('using SDK: ' + kconfig_sdk_root_tree)
else
  message('no SDK declared in cross-file through pkg_config_libdir')
  kconfig_sdk_root_tree = ''
endif

kconfig_env = environment()
kconfig_env.set('KCONFIG_CONFIG', kconfig_dotconfig)
kconfig_env.set('srctree', kconfig_srctree)
kconfig_env.set('subprojects', kconfig_subprojects_tree)
kconfig_env.set('sdk', kconfig_sdk_root_tree)

# Add envvars to devenv to ease developer to use menuconfig directly w/o dedicated target
meson.add_devenv(kconfig_env)

kconfig_py_wrapper = '''
import os
import sys
import subprocess

os.environ["srctree"] = "@0@"
os.environ["subprojects"] = "@1@"
os.environ["sdk"] = "@2@"
res = subprocess.run(args=sys.argv[1:], check=True, capture_output=True, text=True)
print(res.stdout)

'''.format(kconfig_srctree, kconfig_subprojects_tree, kconfig_sdk_root_tree)

# kconfig options is mandatory
assert(fs.is_file(kconfig), 'Please provide a Kconfig file, see meson.options for more informations')

if not fs.is_file(kconfig_dotconfig)
    message('Use @0@ as .config'.format(kconfig_config_in))
    if not fs.is_absolute(kconfig_config_in)
        kconfig_config_in = meson.global_source_root() / kconfig_config_in
    endif
    assert(fs.is_file(kconfig_config_in), 'Please provide a config file, one could generates a default one with:\n alldefconfig @0@'.format(kconfig))

    kconfig_copy_script = '''
import shutil
import sys
from pathlib import Path
orig = Path(sys.argv[1])
oldconfig = Path(sys.argv[2])
config = Path(sys.argv[3])
shutil.copyfile(orig, oldconfig)
shutil.copyfile(oldconfig, config)
'''

    configure_file(
        output: '.config.old',
        command: [py3, '-c', kconfig_copy_script, kconfig_config_in, '@OUTPUT@', kconfig_dotconfig]
    )
endif

# dry run in order to generate dep file only, oldconfig is out of scope of meson configure step
configure_file(
    output: '.config',
    depfile: 'kconfig.deps',
    command: [ py3, '-c', kconfig_py_wrapper, py3, kconfig_py, kconfig, '--dry-run', '@OUTPUT@', '@DEPFILE@', '@OUTPUT@' ],
)

# Create a ksconfig stamp file with current dotconfig as deps.
# Thus, if the dotconfig file changes (manually copy/pasted, after menuconfig, etc.,
# project is reconfigured
kconfig_stamp = configure_file(
    output: '.kconfig.configure.stamp',
    depfile: '.kconfig.configure.deps',
    command: [stamp_configure, '@DEPFILE@', kconfig_dotconfig ],
)

# Convert kconfig entry set to 'y' in 1/0 style boolean
# Thus, boolean kconfig true entry will translate to `#define CONFIG_XXXX 1`
kconfig_data = configuration_data(keyval.load(kconfig_dotconfig))

kconfig_rust_cfg = []
kconfig_rust_env = environment()

foreach key: kconfig_data.keys()
    value = kconfig_data.get_unquoted(key)
    kconfig_rust_cfg += key
    if value == 'y'
        kconfig_data.set10(key, true)
    else
        if value.startswith('0x')
            kconfig_rust_env.set(key + '_STR_HEX', value)
            # XXX:
            #  need a fix in meson to handle base detection based on str prefix 0b, 0o, 0x
            # kconfig_rust_env.set(key, value.to_int())
        else
            kconfig_rust_env.set(key, value)
        endif
    endif
endforeach

# Generates C header from config_data
kconfig_h = configure_file(
    configuration: kconfig_data,
    output: 'generated_kconfig.h'
)

# Generates json file from config_data
kconfig_json = configure_file(
    configuration: kconfig_data,
    output: 'generated_kconfig.json',
    output_format: 'json',
)

# Generates rustarg file from config_data
kconfig_rustargs = configure_file(
    input: 'templates/rustargs.in',
    command: [jinja_cli, '@INPUT@', '--define', 'flags', ' '.join(kconfig_rust_cfg), '-o', '@OUTPUT@'],
    output: 'rustflags.ini',
)

# The followings are helper targets that mimic well known project such as Linux/uboot
# One can edit the current configuration with `ninja menuconfig` within the build dir.
run_target('menuconfig',
    command: [menuconfig, kconfig],
    env: kconfig_env,
)

run_target('oldconfig',
    command: [oldconfig, kconfig],
    env: kconfig_env,
)

run_target('savedefconfig',
    command: [savedefconfig, '--kconfig', kconfig, '--out', kconfig_savedefconfig],
    env: kconfig_env,
)

# about rust crates, to allow meson-bases usage without usint crates.io (e.g. kernel autotest or uapi case)
subdir('rust')
