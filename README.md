# systemctl

Small rust crate to interact with systemd units

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
