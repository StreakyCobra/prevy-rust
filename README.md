![logo][]

_**» Manage your development workspaces with ease.**_

[![Built with Spacemacs](https://cdn.rawgit.com/syl20bnr/spacemacs/442d025779da2f62fc86c2082703697714db6514/assets/spacemacs-badge.svg)](http://spacemacs.org)
[![Build Status](https://travis-ci.org/prevy/prevy.svg?branch=master)](https://travis-ci.org/prevy/prevy)
[![License](https://img.shields.io/badge/license-GPLv3-blue.svg)][LICENSE.md]
[![Join the chat at https://gitter.im/prevy/prevy](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/prevy/prevy?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-generate-toc again -->
**Table of Contents**

- [Presentation](#presentation)
- [Requirements](#requirements)
- [How to build](#how-to-build)
- [Version numbers](#version-numbers)
- [License](#license)

<!-- markdown-toc end -->

# Presentation

`prevy` helps you to manage your development workspaces by displaying status and
acting on all the projects you are working on.

> Author: Fabien Dubosson &lt;<fabien.dubosson@gmail.com>&gt;

# Requirements

- [rust][] (for compilation)
- [cargo][] (for dependencies management)

# How to build

In order to build this project, you will need to install tooling for compiling
[rust][] code. See the [requirements](#requirements) section for the complete
list.

The project can be build with:

    $ cargo build

This will generate a =target/debug/prevy= executable that can be run with the
following command:

    $ ./target/debug/prevy -h

Alternatively, you can directly build and run the project with:

    $ cargo run -q -- -h

# Version numbers

This software uses [Semantic Versioning v2.0.0][semver]. Version numbers are of
the form:

    MAJOR.MINOR.PATCH

- `MAJOR` increases with backwards-incompatible API changes.
- `MINOR` increases with functionality added in a backwards-compatible manner.
- `PATCH` increases with backwards-compatible bug fixes.

Any part may also be incremented at any time if desired.

**Note**: `MAJOR` version `0` is for initial development and does not follow the
`MINOR` and `PATCH` schema. Also do not expect stability nor releases.

# License

This software is licensed under the term of the [GPL v3.0][] license:

    prevy, a software to manage development workspaces with ease.
    Copyright (C) 2016 Fabien Dubosson <fabien.dubosson@gmail.com>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.

See [LICENSE.md][] for the complete license.

[logo]:       ./resources/logo/prevy.png
[rust]:       https://www.rust-lang.org/
[cargo]:      https://crates.io/
[semver]:     http://semver.org/spec/v2.0.0.html
[LICENSE.md]: LICENSE.md
[GPL v3.0]:   https://www.gnu.org/licenses/gpl-3.0.html
