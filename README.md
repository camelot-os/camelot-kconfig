<!--
SPDX-FileCopyrightText: 2024 Ledger SAS
SPDX-License-Identifier: Apache-2.0
-->

# Outpost-Kconfig
Meson Buildsystem recipe collection that handles `Kconfig` and `.config` files.

This meson project can be used as a sub-project for every meson based project which uses `Kconfig` as configuration system. The top level `Kconfig` is parsed at configure time (i.e. during meson setup), the `config` file is checked and a build specific `.config` file is generated using `oldconfig` policy for missing `Kconfig` entry.
During project configuration, two dependencies file are generated, one which listed all `Kconfig` files that are parsed, and the `.config` itself. Thus, if any of those file change (Kconfig new options and/or a manually run menuconfig), the project is automatically reconfigure by `Meson`. This project provides a meson `configuration_data` object containing all `.config` entries (field to `y` are converted to `1/0` boolean) and a `config.h` is generated using meson `configure_file` feature.

The provided `configuration_data` can be reused for Meson `source_set`configuration.

## Options
 - `kconfig`: Top level `Kconfig` file.
 - `config`: input config file.
> **NOTE:** This is up to the user to provide the input config file (e.g. from a `defconfig`, and/or after `config fragment` merge operation). This can be done w/ `scripts/kconfig.py` but this is out of scope of the meson configuration step.

> Options got `yield` attribute and thus can be defined at top-level project and forwarded to sub projects seamlessly

## Environment
`Kconfig` can use environment variables substitution in `kconfig` file and/or `dotconfig`. This recipe export a meson variable named `kconfig_env` to be used as `env` keyword argument value for meson function (see: https://mesonbuild.com/Reference-manual_returned_env.html).

### Exported environment variables
 - `srctree`: standard kconfig variable for top level source directory  (a.k.a. meson global source tree).
 - `subprojects`: helper to path to top level subprojects directory. This ease subprojects `Kconfig` files source from top level `Kconfig`` file.
 - `KCONFIG_CONFIG`: Outputted dotconfig file path.

### meson devenv
`kconfig_env` is also added to the meson devenv. Meson devenv is a python-virtual-env-like feature that allowed developers to enable the required variables for the project. Using this, one can activate the project devenv and directly use `menuconfig`  from `kconfiglib` w/o the need of any helper or custom command.

e.g.

```console
meson devenv -C builddir
menuconfig
```

## Targets (deprecated)
There is a helper target called `menuconfig` for users who wants to edit the configuration during development. This target handles the path of top-level `Kconfig` and output `.config`.

 - `menuconfig`: runs menuconfig inside the given `builddir`
 > **TODO:**  Add oldconfig, savedefconfig targets

## Meson variables
The following variables can be use by top level project to use `Kconfig` features. (see: https://mesonbuild.com/Reference-manual_returned_subproject.html)

- kconfig_env: [environment variables set](#environment)
- kconfig_data: kconfig meson `configure_data` w/ entry set to `y` converted as `1/0 boolean`
- kconfig_h: C header generated at configure time by meson from processed `kconfig_data`
- kconfig_rustargs: Rust flags generated at configure time by meson from processed `kconfig_data`

## Prerequisit
This project uses kconfiglib python package as Kconfig frontend. One could install it with the following:
```console
pip3 install --user kconfiglib
```

# LICENSE
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

 http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

>**NOTE:** This project bundles a modified version `kconfig.py` from [Zephyr project](https://www.zephyrproject.org/), licensed under `ISC`.
