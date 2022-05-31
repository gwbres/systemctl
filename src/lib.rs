//! systemctl: small crate to interact with services through systemctl
//! Homepage: <https://github.com/gwbres/systemctl>
use std::process::ExitStatus;
use std::io::{Read, Error, ErrorKind};
use std::str::FromStr;
use strum_macros::EnumString;

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
        .args(args.clone())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let exitcode = child.wait()?;
    //TODO improve this please
    //match exitcode.success() {
    //    true => {
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
        //},
        /*false => {
            Err(Error::new(ErrorKind::Other,
                format!("/usr/bin/systemctl {:?} failed", args)))
        }*/
    //}
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
    Ok(status.trim_end().eq("active"))
}

/// Returns `true` if given `unit` exists,
/// ie., service could be or is actively deployed
/// and manageable by systemd
pub fn exists (unit: &str) -> std::io::Result<bool> {
    let status = status(unit); 
    Ok(status.is_ok() && !status.unwrap().trim_end().eq(&format!("Unit {}.service could not be found.", unit)))
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

/// `AutoStartStatus` describes the Unit current state 
#[derive(Copy, Clone, PartialEq, Eq, EnumString, Debug)]
pub enum AutoStartStatus {
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "enabled")]
    Enabled,
    #[strum(serialize = "disabled")]
    Disabled,
    #[strum(serialize = "generated")]
    Generated,
    #[strum(serialize = "indirect")]
    Indirect,
}

impl Default for AutoStartStatus {
    fn default() -> AutoStartStatus { AutoStartStatus::Disabled }
}

/// `Type` describes a Unit declaration Type in systemd
#[derive(Copy, Clone, PartialEq, Eq, EnumString, Debug)]
pub enum Type {
    #[strum(serialize = "automount")]
    AutoMount,
    #[strum(serialize = "mount")]
    Mount,
    #[strum(serialize = "service")]
    Service,
    #[strum(serialize = "scope")]
    Scope,
    #[strum(serialize = "socket")]
    Socket,
    #[strum(serialize = "slice")]
    Slice,
    #[strum(serialize = "timer")]
    Timer,
    #[strum(serialize = "path")]
    Path,
    #[strum(serialize = "target")]
    Target,
}

impl Default for Type {
    fn default() -> Type { Type::Service }
}

/// `State` describes a Unit current state 
#[derive(Copy, Clone, PartialEq, Eq, EnumString, Debug)]
pub enum State {
    #[strum(serialize = "masked")]
    Masked,
    #[strum(serialize = "loaded")]
    Loaded,
}

impl Default for State {
    fn default() -> State { State::Masked }
}

/// Process
#[derive(Clone, Debug)]
pub struct Process {
    /// pid
    pid: u64,
    /// command line that was executed
    command: String,
    /// code
    code: String,
    /// status
    status: String,
}

impl Default for Process {
    fn default() -> Process {
        Process {
            pid: 0,
            command: Default::default(),
            code: Default::default(),
            status: Default::default(),
        }
    }
}

/// Structure to describe a systemd `unit`
#[derive(Clone, Debug)]
pub struct Unit {
    /// Unit name
    pub name: String,
    /// Unit type
    pub utype: Type,
    /// Optionnal unit description
    pub description: Option<String>,
    /// Current state
    pub state: State,
    /// Auto start feature
    pub auto_start: AutoStartStatus,
    /// `true` if Self is actively running
    pub active: bool,
    /// `true` if this unit is auto started by default,
    /// meaning, it should be manually disabled 
    /// not to automatically start
    pub preset: bool,
    /// Configuration script loaded when starting this unit
    pub script: String,
    /// Optionnal process description
    pub process: Option<String>,
    /// Current PID 
    pub pid: Option<u64>,
    /// Running task(s) infos
    pub tasks: Option<String>,
    /// Memory consumption infos
    pub memory: Option<String>,
    /// mounted partition (`What`), if this is a `mount`/`automount` unit
    pub mounted: Option<String>,
    /// Mount point (`Where`), if this is a `mount`/`automount` unit
    pub mountpoint: Option<String>,
    /// Docs / `man` page(s) available for this unit
    pub docs: Option<Vec<String>>,
}

impl Default for Unit {
    /// Builds a default `Unit` structure
    fn default() -> Unit {
        Unit {
            name: Default::default(), 
            utype: Default::default(),
            description: Default::default(),
            script: Default::default(),
            pid: Default::default(),
            tasks: Default::default(),
            memory: Default::default(),
            state: Default::default(),
            auto_start: Default::default(),
            preset: Default::default(),
            active: Default::default(),
            docs: Default::default(),
            process: Default::default(),
            mounted: Default::default(),
            mountpoint: Default::default(),
        }
    }
}

impl Unit {
    /// Builds a new `Unit` structure by retrieving 
    /// structure attributes with a `systemctl status $unit` call
    pub fn from_systemctl (name: &str) -> std::io::Result<Unit> {
        let status = status(name)?;
        let mut lines = status.lines();
        let next = lines.next().unwrap();
        let (_, rem) = next.split_at(3); 
        let mut items = rem.split_ascii_whitespace();
        let name = items.next().unwrap().trim();
        let mut description : Option<String> = None;
        if let Some(delim) = items.next() {
            if delim.trim().eq("-") {
                // --> description string is provided
                let items : Vec<_> = items.collect();
                description = Some(itertools::join(&items, " "));
            }
        }
        let items : Vec<_> = name.split_terminator(".").collect();
        let name = items[0]; 
        // `type` is deduced from .extension
        let utype = Type::from_str(items[1].trim()).unwrap(); 
        let mut script: String = String::new();
        let mut process: Option<Process> = None;
        let mut pid : Option<u64> = None;
        let mut state: State = State::default();
        let mut auto_start : AutoStartStatus = AutoStartStatus::default();
        let mut active: bool = false;
        let mut preset: bool = false;
        let mut memory: Option<String> = None;
        let mut mounted: Option<String> = None;
        let mut mountpoint: Option<String> = None;
        let mut docs: Option<Vec<String>> = None;
        for line in lines {
            let line = line.trim_start();
            if line.starts_with("Loaded:") {
                let (_, line) = line.split_at(8); // "Loaded: "
                if line.starts_with("loaded") {
                    state = State::Loaded;
                    let (_, rem) = line.split_at(1); // "("
                    let (rem, _) = rem.split_at(rem.len()-1); // ")"
                    let items : Vec<_> = rem.split_terminator(";").collect();
                    script = items[0].trim().to_string();
                    auto_start = AutoStartStatus::from_str(items[1].trim()).unwrap();
                    if items.len() > 2 {
                        // preset is optionnal ?
                        preset = items[2].trim().ends_with("enabled") 
                    }
                } else if line.starts_with("masked") {
                    state = State::Masked;
                }
            
            } else if line.starts_with("Active: ") {
                //LINE: "Active: active (running) since Fri 2022-03-04 08:29:34 CET; 1 months 8 days ago"
            
            } else if line.starts_with("Docs: ") {
                //LINE: "Docs: man:sshd(8)"
                //LINE: "man:sshd_config(5)"
            
            } else if line.starts_with("What: ") {
                mounted = Some(line.split_at(6).1.trim().to_string());
            } else if line.starts_with("Where: ") {
                mountpoint = Some(line.split_at(7).1.trim().to_string());

            } else if line.starts_with("Main PID: ") {
                let items : Vec<_> = line.split_ascii_whitespace().collect();
                pid = Some(u64::from_str_radix(items[2].trim(), 10).unwrap());
            
            } else if line.starts_with("Process: ") {
                let items : Vec<_> = line.split_ascii_whitespace().collect();
                let proc_pid = u64::from_str_radix(items[1].trim(), 10).unwrap();
                //let cli;
                //Process: 640 ExecStartPre=/usr/sbin/sshd -t (code=exited, status=0/SUCCESS)

            } else if line.starts_with("CGroup: ") {
                //LINE: "CGroup: /system.slice/sshd.service"
                //LINE: "└─1050 /usr/sbin/sshd -D"
            } else if line.starts_with("Tasks: ") {

            } else if line.starts_with("Memory: ") {
                let line = line.split_at(8).1;
                memory = Some(line.to_string())
            }
        }
        Ok(Unit {
            name: name.to_string(),
            description, 
            script,
            utype, 
            pid,
            state,
            auto_start,
            preset,
            active,
            tasks: Default::default(),
            process: Default::default(),
            memory,
            mounted,
            mountpoint,
            docs,
        })
    }
    /// Restarts Self by invocking `systemctl`
    pub fn restart (&self) -> std::io::Result<ExitStatus> {
        restart(&self.name)
    }
    /// Returns verbose status for Self 
    pub fn status (&self) -> std::io::Result<String> {
        status(&self.name)
    }
    /// Returns `true` if Self is actively running
    pub fn is_active (&self) -> std::io::Result<bool> {
        is_active(&self.name)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_status() {
        let status = status("sshd");
        assert_eq!(status.is_ok(), true);
        println!("sshd status : {:#?}", status)
    }
    #[test]
    fn test_is_active() {
        let units = vec!["sshd","dropbear","ntpd"];
        for u in units {
            let active = is_active(u);
            assert_eq!(active.is_ok(), true);
            println!("{} is-active: {:#?}", u, active);
        }
    }
    #[test]
    fn test_service_exists() {
        let units = vec!["sshd","dropbear","ntpd","example","non-existing","dummy"];
        for u in units {
            let ex = exists(u);
            assert_eq!(ex.is_ok(), true);
            println!("{} exists: {:#?}", u, ex);
        }
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
    fn test_service_unit_construction() {
        let units = list_units(None, None).unwrap(); // all units
        for unit in units {
            let unit = unit.as_str();
            if unit.contains("@") {
                // not testing this one
                // would require @x service # identification / enumeration
                continue
            }
            let c0 = unit.chars().nth(0).unwrap();
            if c0.is_alphanumeric() { // valid unit name --> run test
                let u = Unit::from_systemctl(&unit).unwrap();
                println!("####################################");
                println!("Unit: {:#?}", u);
                println!("active: {}", u.active);
                println!("preset: {}", u.preset);
                println!("auto_start (enabled): {:#?}", u.auto_start);
                println!("config script : {}", u.script);
                println!("pid: {:?}", u.pid);
                println!("Running task(s): {:?}", u.tasks);
                println!("Memory consumption: {:?}", u.memory);
                println!("####################################")
            }
        }
    }
}
