This crate provides a derive macro to add encryption/decryption functionality to any serializable struct.
It is built on top of [`strong-box`](https://docs.rs/strong-box), which means you get safe, fast, and modern cryptography without compromising on useability.

See [the docs](https://docs.rs/struct-box) for usage instructions (it's designed to be as simple as possible).

# MSRV

Specified in `Cargo.toml`.
Bumping is a breaking change.

# Licence

Unless otherwise stated, everything in this repo is covered by the following
copyright notice:

```text
    Copyright (C) 2024  Matt Palmer <matt@hezmatt.org>

    This program is free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License version 3, as
    published by the Free Software Foundation.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
```
