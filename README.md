# systemctl

Small rust crate to interact with systemd units

[![crates.io](https://img.shields.io/crates/v/systemctl.svg)](https://crates.io/crates/systemctl)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/systemctl/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/systemctl/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/systemctl.svg)](https://crates.io/crates/systemctl)   
[![Rust](https://github.com/gwbres/systemctl/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/systemctl/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/systemctl/badge.svg)](https://docs.rs/systemctl/badge.svg)

## Environment

`SYSTEMCTL_PATH` custom env. variable describes the absolute
location path of `systemctl` binary, by default this crate uses `/usr/bin/systemctl`,
but that can be customized.

## Unit / service operation

Nominal service operations:

```rust
systemctl::stop("systemd-journald.service")
    .unwrap();
systemctl::restart("systemd-journald.service")
    .unwrap();

if let Ok(true) = systemctl::exists("ntpd") {
    let is_active = systemctl::is_active("ntpd")
        .unwrap();
}
```

## Service enumeration

```rust
use systemctl;
// list all units
systemctl::list_units(None, None);

// list all services 
// by adding a --type filter
systemctl::list_units(Some("service"), None);

// list all services currently `enabled` 
// by adding a --state filter
systemctl::list_units(Some("service"), Some("enabled"));
```

## Unit structure

Use the unit structure for more information

```rust
let unit = systemctl::Unit::from_systemctl("sshd")
    .unwrap();
unit.restart().unwrap();
println!("active: {}", unit.active);
println!("preset: {}", unit.preset);

if let Some(docs) = unit.docs { // doc pages available
    for doc in docs {
        if let Some(page) = doc.as_man() {
            // `man` page exists 
        }
        if let Some(url) = doc.as_url() {
            // `url` is indicated
        }
    }
}

println!("auto_start (enabled): {}", unit.auto_start);
println!("config script : {}", unit.script);
println!("pid: {}", unit.pid);
println!("Running task(s): {}", unit.tasks.unwrap());
println!("Memory consumption: {}", unit.memory.unwrap());
```

## TODO

* [ ] parse all known attributes in `from_systemctl`
* [ ] currently we study (try to parse) stdio result from systemctl() invokation,
without testing the exit status, which is bad. This is due to the fact
that we get apparently get an error code, on existing units that currently have a `dead` state
