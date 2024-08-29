# openalias.rs [![AppVeyorCI build status](https://ci.appveyor.com/api/projects/status/cspjknvfow5gfro0/branch/master?svg=true)](https://ci.appveyor.com/project/nabijaczleweli/openalias-rs/branch/master) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE) [![Crates.io version](https://img.shields.io/crates/v/openalias)](https://crates.io/crates/openalias)
Look up and parse [OpenAlias](https://openalias.org) data.

## [Documentation](https://rawcdn.githack.com/nabijaczleweli/openalias.rs/doc/openalias/index.html)
## [Manpage](https://rawcdn.githack.com/nabijaczleweli/openalias.rs/man/openalias.1.html)

### Usage

Get cryptocurrency addresses for "nabijaczleweli.xyz" and "donate.getmonero.org" aliases:

```sh
openalias nabijaczleweli.xyz donate.getmonero.org
```

Look up and get BTC address for "donate@nabijaczleweli.xyz":

```rust
extern crate openalias;
openalias::addresses("donate@nabijaczleweli.xyz");
```

For more information and examples see the [manpage](https://rawcdn.githack.com/nabijaczleweli/openalias.rs/man/openalias.1.html) and/or
[documentation](https://rawcdn.githack.com/nabijaczleweli/openalias.rs/doc/openalias/index.html).

## Lookup leak protection with DNSCrypt

Just use [dnscrypt-proxy](https://github.com/jedisct1/dnscrypt-proxy) on your system.
