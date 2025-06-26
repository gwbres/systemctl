# systemctl

Small rust crate to interact with systemd units through `systemctl`.

At the time I needed those features, I was not aware of `zbus-systemd`, which is now available and should be prefered.  
This crate uses `systemctl` interaction directly, which is far from ideal for applications.
`zbus-systemd` should therefore be prefered.

[![crates.io](https://img.shields.io/crates/v/systemctl.svg)](https://crates.io/crates/systemctl)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/systemctl/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/systemctl/blob/main/LICENSE-MIT)
[![crates.io](https://img.shields.io/crates/d/systemctl.svg)](https://crates.io/crates/systemctl)  
[![Rust](https://github.com/gwbres/systemctl/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/systemctl/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/systemctl/badge.svg)](https://docs.rs/systemctl/badge.svg)

## Features

* serde: Enable to make structs in this crate De-/Serializable

## Limitations

Currently, systemd Version <245 are not supported as unit-file-list changed from two column to three column setup. See: [systemd Changelog](https://github.com/systemd/systemd/blob/16bfb12c8f815a468021b6e20871061d20b50f57/NEWS#L6073)

## Unit / service operation

Nominal service operations:

```rust
let systemctl = systemctl::SystemCtl::default();
systemctl.stop("systemd-journald.service")
    .unwrap();
systemctl.restart("systemd-journald.service")
    .unwrap();

if let Ok(true) = systemctl.exists("ntpd") {
    let is_active = systemctl.is_active("ntpd")
        .unwrap();
}
```

## Service enumeration

```rust
let systemctl = systemctl::SystemCtl::default();
// list all units
systemctl.list_units(None, None, None);

// list all services 
// by adding a --type filter
systemctl.list_units(Some("service"), None, None);

// list all services currently `enabled` 
// by adding a --state filter
systemctl.list_units(Some("service"), Some("enabled"), None);

// list all services starting with cron
systemctl.list_units(Some("service"), None, Some("cron*"));

// Check if a unit is active
systemctl.get_active_state("service");

// list dependencies of a service or target
systemctl.list_dependencies("some.target");
```

## Unit structure

Use the unit structure for more information

```rust
let systemctl = systemctl::SystemCtl::default();
let unit = systemctl.create_unit("ssh.service")
    .unwrap();
systemctl.restart(&unit.name).unwrap();
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

println!("auto_start (enabled): {:?}", unit.auto_start);
println!("config script : {}", unit.script);
println!("pid: {:?}", unit.pid);
println!("Running task(s): {:?}", unit.tasks);
println!("Memory consumption: {:?}", unit.memory);
```

## TODO

* [ ] parse all known attributes in `from_systemctl`
