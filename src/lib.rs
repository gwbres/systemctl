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
pub fn restart (unit: &str) -> std::io::Result<ExitStatus> { systemctl(vec!["restart", unit]) }

/// Forces given `unit` to stop
pub fn stop (unit: &str) -> std::io::Result<ExitStatus> { systemctl(vec!["stop", unit]) }

/// Returns raw status from `systemctl status $unit` call
pub fn status (unit: &str) -> std::io::Result<String> { systemctl_capture(vec!["status", unit]) }

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
        if parsed.len() == 2 {
            result.push(parsed[0].to_string())
        }
    }
    Ok(result)
}

/// Returns list of services that are currently declared as disabled
pub fn list_disabled_services() -> std::io::Result<Vec<String>> { Ok(list_units(Some("service"), Some("disabled"))?) }

/// Returns list of services that are currently declared as enabled
pub fn list_enabled_services() -> std::io::Result<Vec<String>> { Ok(list_units(Some("service"), Some("enabled"))?) }

/// `State` describes a Unit State in systemd
#[derive(Copy, Clone, Debug)]
pub enum State {
    Static,
    Indirect,
    Enabled,
    Disabled,
}

impl Default for State {
    fn default() -> State { State::Disabled }
}

/// `Type` describes a Unit declaration Type in systemd
#[derive(Copy, Clone, Debug)]
pub enum Type {
    Mount,
    Service,
    Scope,
    Socket,
    Slice,
    Timer,
}

impl Default for Type {
    fn default() -> Type { Type::Service }
}

/// Structure to describe a systemd `unit`
pub struct Unit {
    /// Unit name
    pub name: String,
    /// Unit type
    pub utype: Type,
    /// Unit description string
    pub description: String,
    /// Configuration script loaded when starting this unit
    pub script: String,
    /// Systemd declaration
    pub state: State,
}

impl Default for Unit {
    /// Builds a default `Unit` structure
    fn default() -> Unit {
        Unit {
            name: Default::default(), 
            utype: Default::default(),
            description: Default::default(),
            script: Default::default(),
            state: Default::default(),
        }
    }
}

impl Unit {
    /// Builds a new descriptor for desired `unit`
    pub fn new (name: &str, unit_type: Type, description: &str, script: &str, state: State) -> Unit {
        Unit {
            name: name.to_string(),
            script: script.to_string(),
            description: description.to_string(),
            utype: unit_type,
            state: state,
        }
    }
    /// Builds a new `Unit` structure by retrieving 
    /// structure attributes with a `systemctl status $unit` call
    pub fn from_systemctl (name: &str) -> std::io::Result<Unit> {
        let status = status(name)?;
        let mut lines = status.lines();
        // line[0] : xxxx.type - description
        let next = lines.next().unwrap();
        let (_, rem) = next.split_at(3); 
        let mut content = rem.split_terminator("-");
        let name = content.next().unwrap().trim();
        let descriptor = content.next().unwrap().trim();
        let mut content = name.split_terminator(".");
        let name = content.next().unwrap();
        let utype = content.next().unwrap();
        for line in lines {
            let line = line.trim_start();
            println!("LINE: \"{}\"", line);
            if line.starts_with("Loaded:") {
                //LINE: "Loaded: loaded (/usr/lib/systemd/system/sshd.service; enabled; vendor preset: enabled)"
                let line = line.split_at(7+9); // "Loader: " + "loaded_("
            
            } else if line.starts_with("Active: ") {
                //LINE: "Active: active (running) since Fri 2022-03-04 08:29:34 CET; 1 months 8 days ago"
            
            } else if line.starts_with("Docs: ") {
                //LINE: "Docs: man:sshd(8)"
                //LINE: "man:sshd_config(5)"

            } else if line.starts_with("Main PID: ") {
                //LINE: "Main PID: 1050 (sshd)"

            } else if line.starts_with("CGroup: ") {
                //LINE: "CGroup: /system.slice/sshd.service"
                //LINE: "└─1050 /usr/sbin/sshd -D"
            }
        }
        println!("Name: {} - Type: {} - Descriptor: {}", name, utype, descriptor);
        Ok(Unit {
            name: name.to_string(),
            description: descriptor.to_string(),
            script: Default::default(),
            state: Default::default(),
            utype: Default::default(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_status() {
        let status = status("sshd");
        println!("sshd status : {:#?}", status)
    }
    #[test]
    fn test_is_active() {
        let active = is_active("sshd").unwrap();
        println!("sshd active: {:#?}", active)
    }
    #[test]
    fn test_disabled_services() {
        let services = list_disabled_services().unwrap();
        println!("disabled services: {:#?}", services)
    }
    #[test]
    fn test_enabled_services() {
        let services = list_enabled_services().unwrap();
        println!("enabled services: {:#?}", services)
    }
    #[test]
    fn test_unit_construction() {
        let _ = Unit::from_systemctl("sshd").unwrap();
    }
}
