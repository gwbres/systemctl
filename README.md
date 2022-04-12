# systemctl

Small rust crate to interact with systemd units

[![crates.io](https://img.shields.io/crates/v/systemctl.svg)](https://crates.io/crates/systemctl)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/systemctl/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/systemctl/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/dms-coordinates.svg)](https://crates.io/crates/systemctl)   
[![Rust](https://github.com/gwbres/systemctl/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gwbres/systemctl/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/systemctl/badge.svg)](https://docs.rs/systemctl/badge.svg)

## Unit / service operation

Nominal service operations:

```rust
use systemctl;
systemctl::stop("systemd-journald.service").unwrap();
systemctl::restart("systemd-journald.service").unwrap();
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
