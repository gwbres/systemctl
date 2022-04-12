//! systemctl: small crate to interact with services through systemctl
//! Homepage: <https://github.com/gwbres/systemctl>
use std::process::ExitStatus;
use std::io::{Read, Error, ErrorKind};

/// calls systemctl $args
fn systemctl (args: Vec<&str>) -> std::io::Result<ExitStatus> {
    let mut child = std::process::Command::new("/usr/bin/systemctl")
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    child.wait()
}

/// calls systemctl $args and captures stdout
fn systemctl_capture (args: Vec<&str>) -> std::io::Result<String> {
    let mut child = std::process::Command::new("/usr/bin/systemctl")
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let exitcode = child.wait()?;
    match exitcode.success() {
        true => {
            let mut stdout : Vec<u8> = Vec::new();
            if let Ok(size) = child.stdout.unwrap().read_to_end(&mut stdout) {
                if size > 0 {
                    if let Ok(s) = String::from_utf8(stdout) {
                        Ok(s)
                    } else {
                        Err(Error::new(ErrorKind::InvalidData, "Invalid utf8 data in stdout"))
                    }
                } else {
                    Err(Error::new(ErrorKind::InvalidData, "systemctl stdout empty"))
                }
            } else {
                Err(Error::new(ErrorKind::InvalidData, "systemctl stdout empty"))
            }
        },
        false => {
            Err(Error::new(ErrorKind::Other, "systemctl call failed"))
        }
    }
}

/// Forces given `unit` (re)start
pub fn restart (unit: &str) -> std::io::Result<ExitStatus> { systemctl(vec![unit, "restart"]) }

/// Forces given `unit` to stop
pub fn stop (unit: &str) -> std::io::Result<ExitStatus> { systemctl(vec![unit, "stop"]) }

/// Returns raw status from `systemctl status $unit` call
pub fn status (unit: &str) -> std::io::Result<String> { systemctl_capture(vec![unit, "status"]) }

/// Returns `true` if given `unit` is actively running
pub fn is_active (unit: &str) -> std::io::Result<bool> {
    let status = systemctl_capture(vec!["is-active", unit])?;
    Ok(status.contains("Active: active (running)"))
}

/// Returns list of units extracted from systemctl listing.  
///  + type filter: optionnal --type filter
///  + state filter: optionnal --state filter
fn list_units (type_filter: Option<&str>, state_filter: Option<&str>) -> std::io::Result<Vec<String>> {
    let mut args = vec!["list-unit-files"];
    if let Some(filter) = type_filter {
        args.push("--type");
        args.push(filter)
    }
    if let Some(filter) = state_filter {
        args.push("--state");
        args.push(filter)
    }
    let mut result : Vec<String> = Vec::new();
    let content = systemctl_capture(args)?;
    let lines = content.lines();
    for l in lines.skip(1) { // header labels
        let parsed : Vec<_> = l.split_ascii_whitespace().collect();
        result.push(parsed[0].to_string())
    }
    Ok(result)
}

/// Returns list of services that are currently declared as disabled
pub fn list_disabled_services() -> std::io::Result<Vec<String>> { Ok(list_units(Some("service"), Some("disabled"))?) }

/// Returns list of services that are currently declared as enabled
pub fn list_enabled_services() -> std::io::Result<Vec<String>> { Ok(list_units(Some("service"), Some("enabled"))?) }

/*
/// `State` describes a Unit State in systemd
pub State {
    Static,
    Indirect,
    Enabled,
    Disabled,
}

/// Structure to describe a systemd `unit`
struct Unit {
    /// Unit name
    name: String, 
    /// Service script loaded when starting this unit
    script: String,
    /// Current service status
    active: bool,
    /// `true` if this unit is automatically started
    enabled: bool,
}

impl Default for Unit {
    /// Builds a default `Unit` structure
    fn default() -> Unit {
        Unit {
            name: Default::default(), 
            script: Default::default(),
            active: Default::default(),
            enabled: Default::default(),
        }
    }
}

impl Unit {
    /// Builds a new descriptor for desired `unit`
    pub fn new (name: &str, script: &str, active: bool, enabled: bool) -> Unit {
        Unit {
            name: name.to_string(),
            script: script.to_string(),
            active,
            enabled,
        }
    }
    /// Builds a new `Unit` structure by retrieving 
    /// structure attributes with a `systemctl status $unit` call
    pub fn from_systemctl (name: &str) -> std::io::Result<Unit> {
        let status = status(name)?;
        let mut stdout : Vec<u8> = Vec::new();
        if let Ok(_) = status.stdout.unwrap().read_to_end(&mut stdout) {
            if let Ok(content) = String::from_utf8(stdout) {
                let mut lines = content.lines();
                let next = lines.next();
                let (_, rem) = 
                Ok(Unit::default())
            } else {
                Err(Error::new(ErrorKind::InvalidData, "Invalid utf8 data in stdout"))
            }
        } else {
            Err(Error::new(ErrorKind::InvalidData, "systemctl stdout is empty"))
        }
    }
}

//}
/// ● arp-ethers.service - Load static arp entries
///    Loaded: loaded (/usr/lib/systemd/system/arp-ethers.service; disabled; vendor preset:
///    disabled)
///       Active: inactive (dead)
///            Docs: man:arp(8)
///                       man:ethers(5)
///
///╰─$ systemctl status tuned.service
///1 ↵
///● tuned.service - Dynamic System Tuning Daemon
///   Loaded: loaded (/usr/lib/systemd/system/tuned.service; enabled; vendor preset: enabled)
///      Active: active (running) since Fri 2022-03-04 08:29:39 CET; 1 months 8 days ago
///           Docs: man:tuned(8)
///                      man:tuned.conf(5)
///                                 man:tuned-adm(8)
///                                  Main PID: 1053 (tuned)
///                                     CGroup: /system.slice/tuned.service
///                                                └─1053 /usr/bin/python2 -Es /usr/sbin/tuned -l
///                                                -P
*/
