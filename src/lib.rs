//! Crate to manage and monitor services through `systemctl`   
//! Homepage: <https://github.com/gwbres/systemctl>
use std::io::{Error, ErrorKind, Read};
use std::process::ExitStatus;
use std::str::FromStr;
use strum_macros::EnumString;

#[macro_use]
extern crate default_env;

/// Invokes `systemctl $args` silently
fn systemctl(args: Vec<&str>) -> std::io::Result<ExitStatus> {
    let mut child =
        std::process::Command::new(default_env!("SYSTEMCTL_PATH", "/usr/bin/systemctl"))
            .args(args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()?;
    child.wait()
}

/// Invokes `systemctl $args` and captures stdout stream
fn systemctl_capture(args: Vec<&str>) -> std::io::Result<String> {
    let mut child =
        std::process::Command::new(default_env!("SYSTEMCTL_PATH", "/usr/bin/systemctl"))
            .args(args.clone())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()?;
    let _exitcode = child.wait()?;
    //TODO: improve this please
    //Interrogating some services returns an error code
    //match exitcode.success() {
    //true => {
    let mut stdout: Vec<u8> = Vec::new();
    if let Ok(size) = child.stdout.unwrap().read_to_end(&mut stdout) {
        if size > 0 {
            if let Ok(s) = String::from_utf8(stdout) {
                Ok(s)
            } else {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid utf8 data in stdout",
                ))
            }
        } else {
            Err(Error::new(ErrorKind::InvalidData, "systemctl stdout empty"))
        }
    } else {
        Err(Error::new(ErrorKind::InvalidData, "systemctl stdout empty"))
    }
    /*},
        false => {
            Err(Error::new(ErrorKind::Other,
                format!("/usr/bin/systemctl {:?} failed", args)))
        }
    }*/
}

/// Forces given `unit` to (re)start
pub fn restart(unit: &str) -> std::io::Result<ExitStatus> {
    systemctl(vec!["restart", unit])
}

/// Forces given `unit` to stop
pub fn stop(unit: &str) -> std::io::Result<ExitStatus> {
    systemctl(vec!["stop", unit])
}

/// Returns raw status from `systemctl status $unit` call
pub fn status(unit: &str) -> std::io::Result<String> {
    systemctl_capture(vec!["status", unit])
}

/// Invokes systemctl `cat` on given `unit`
pub fn cat(unit: &str) -> std::io::Result<String> {
    systemctl_capture(vec!["cat", unit])
}

/// Returns `true` if given `unit` is actively running
pub fn is_active(unit: &str) -> std::io::Result<bool> {
    let status = systemctl_capture(vec!["is-active", unit])?;
    Ok(status.trim_end().eq("active"))
}

/// Isolates given unit, only self and its dependencies are
/// now actively running
pub fn isolate(unit: &str) -> std::io::Result<ExitStatus> {
    systemctl(vec!["isolate", unit])
}

/// Freezes (halts) given unit.
/// This operation might not be feasible.
pub fn freeze(unit: &str) -> std::io::Result<ExitStatus> {
    systemctl(vec!["freeze", unit])
}

/// Unfreezes given unit (recover from halted state).
/// This operation might not be feasible.
pub fn unfreeze(unit: &str) -> std::io::Result<ExitStatus> {
    systemctl(vec!["thaw", unit])
}

/// Returns `true` if given `unit` exists,
/// ie., service could be or is actively deployed
/// and manageable by systemd
pub fn exists(unit: &str) -> std::io::Result<bool> {
    let unit_list = match list_units(None, None, Some(unit)) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };
    Ok(!unit_list.is_empty())
}

/// Returns list of units extracted from systemctl listing.   
///  + type filter: optional --type filter
///  + state filter: optional --state filter
///  + glob filter: optional for unit name
pub fn list_units_full(
    type_filter: Option<&str>,
    state_filter: Option<&str>,
    glob: Option<&str>,
) -> std::io::Result<Vec<SmallUnitList>> {
    let mut args = vec!["list-unit-files"];
    if let Some(filter) = type_filter {
        args.push("--type");
        args.push(filter)
    }
    if let Some(filter) = state_filter {
        args.push("--state");
        args.push(filter)
    }
    if let Some(glob) = glob {
        args.push(glob)
    }
    let content = systemctl_capture(args)?;
    let lines = content
        .lines()
        .filter(|line| line.contains('.') && !line.ends_with('.'));

    let mut res_vec: Vec<SmallUnitList> = Vec::new();

    for line in lines {
        let parsed: Vec<&str> = line.split_ascii_whitespace().collect();
        let vendor_preset = match parsed[2] {
            "-" => None,
            "enabled" => Some(true),
            "disabled" => Some(false),
            _ => Some(false),
        };
        res_vec.push(SmallUnitList {
            unit_file: parsed[0].to_string(),
            state: parsed[1].to_string(),
            vendor_preset,
        })
    }

    Ok(res_vec)
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct SmallUnitList {
    pub unit_file: String,
    pub state: String,
    pub vendor_preset: Option<bool>,
}

pub fn list_units(
    type_filter: Option<&str>,
    state_filter: Option<&str>,
    glob: Option<&str>,
) -> std::io::Result<Vec<String>> {
    let list = list_units_full(type_filter, state_filter, glob);
    Ok(list?.iter().map(|n| n.unit_file.clone()).collect())
}

/// Returns list of services that are currently declared as disabled
pub fn list_disabled_services() -> std::io::Result<Vec<String>> {
    list_units(Some("service"), Some("disabled"), None)
}

/// Returns list of services that are currently declared as enabled
pub fn list_enabled_services() -> std::io::Result<Vec<String>> {
    list_units(Some("service"), Some("enabled"), None)
}

/// `AutoStartStatus` describes the Unit current state
#[derive(Copy, Clone, PartialEq, Eq, EnumString, Debug, Default)]
pub enum AutoStartStatus {
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "enabled")]
    Enabled,
    #[strum(serialize = "enabled-runtime")]
    EnabledRuntime,
    #[strum(serialize = "disabled")]
    #[default]
    Disabled,
    #[strum(serialize = "generated")]
    Generated,
    #[strum(serialize = "indirect")]
    Indirect,
    #[strum(serialize = "transient")]
    Transient,
}

/// `Type` describes a Unit declaration Type in systemd
#[derive(Copy, Clone, PartialEq, Eq, EnumString, Debug, Default)]
pub enum Type {
    #[strum(serialize = "automount")]
    AutoMount,
    #[strum(serialize = "mount")]
    Mount,
    #[strum(serialize = "service")]
    #[default]
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

/// `State` describes a Unit current state
#[derive(Copy, Clone, PartialEq, Eq, EnumString, Debug, Default)]
pub enum State {
    #[strum(serialize = "masked")]
    #[default]
    Masked,
    #[strum(serialize = "loaded")]
    Loaded,
}

/*
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
}*/

/// Doc describes types of documentation possibly
/// available for a systemd `unit`
#[derive(Clone, Debug)]
pub enum Doc {
    /// Man page is available
    Man(String),
    /// Webpage URL is indicated
    Url(String),
}

impl Doc {
    /// Unwrapps self as `Man` page
    pub fn as_man(&self) -> Option<&str> {
        match self {
            Doc::Man(s) => Some(s),
            _ => None,
        }
    }
    /// Unwrapps self as webpage `Url`
    pub fn as_url(&self) -> Option<&str> {
        match self {
            Doc::Url(s) => Some(s),
            _ => None,
        }
    }
}

impl std::str::FromStr for Doc {
    type Err = std::io::Error;
    /// Builds `Doc` from systemd status descriptor
    fn from_str(status: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = status.split(':').collect();
        if items.len() != 2 {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "malformed doc descriptor",
            ));
        }
        match items[0] {
            "man" => {
                let content: Vec<&str> = items[1].split('(').collect();
                Ok(Doc::Man(content[0].to_string()))
            },
            "http" => Ok(Doc::Url("http:".to_owned() + items[1].trim())),
            "https" => Ok(Doc::Url("https:".to_owned() + items[1].trim())),
            _ => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "unknown type of doc",
            )),
        }
    }
}

/// Structure to describe a systemd `unit`
#[derive(Clone, Debug, Default)]
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
    /// restart policy
    pub restart_policy: Option<String>,
    /// optionnal killmode info
    pub kill_mode: Option<String>,
    /// Optionnal process description (main tasklet "name")
    pub process: Option<String>,
    /// Optionnal process ID number (main tasklet pid)
    pub pid: Option<u64>,
    /// Running task(s) infos
    pub tasks: Option<u64>,
    /// Optionnal CPU load consumption infos
    pub cpu: Option<String>,
    /// Optionnal Memory consumption infos
    pub memory: Option<String>,
    /// mounted partition (`What`), if this is a `mount`/`automount` unit
    pub mounted: Option<String>,
    /// Mount point (`Where`), if this is a `mount`/`automount` unit
    pub mountpoint: Option<String>,
    /// Docs / `man` page(s) available for this unit
    pub docs: Option<Vec<Doc>>,
    /// wants attributes: list of other service / unit names
    pub wants: Option<Vec<String>>,
    /// wanted_by attributes: list of other service / unit names
    pub wanted_by: Option<Vec<String>>,
    /// also attributes
    pub also: Option<Vec<String>>,
    /// `before` attributes
    pub before: Option<Vec<String>>,
    /// `after` attributes
    pub after: Option<Vec<String>>,
    /// exec_start attribute: actual command line
    /// to be exected on `start` requests
    pub exec_start: Option<String>,
    /// exec_reload attribute, actual command line
    /// to be exected on `reload` requests
    pub exec_reload: Option<String>,
    /// If a command is run as transient service unit, it will be started and managed
    /// by the service manager like any other service, and thus shows up in the output
    /// of systemctl list-units like any other unit.
    pub transient: bool,
}

// TODO: Remove this lint fix
#[allow(clippy::if_same_then_else)]
impl Unit {
    /// Builds a new `Unit` structure by retrieving
    /// structure attributes with a `systemctl status $unit` call
    pub fn from_systemctl(name: &str) -> std::io::Result<Unit> {
        if let Ok(false) = exists(name) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unit or service \"{}\" does not exist", name),
            ));
        }
        let mut u = Unit::default();
        let status = status(name)?;
        let mut lines = status.lines();
        let next = lines.next().unwrap();
        let (_, rem) = next.split_at(3);
        let mut items = rem.split_ascii_whitespace();
        let name = items.next().unwrap().trim();
        if let Some(delim) = items.next() {
            if delim.trim().eq("-") {
                // --> description string is provided
                let items: Vec<_> = items.collect();
                u.description = Some(itertools::join(&items, " "));
            }
        }
        let items: Vec<_> = name.split_terminator('.').collect();
        let name = items[0];

        // `type` is deduced from .extension
        u.utype = Type::from_str(items[1].trim()).unwrap();
        let mut docs: Vec<Doc> = Vec::with_capacity(3);
        let mut is_doc = false;
        for line in lines {
            let line = line.trim_start();
            if let Some(line) = line.strip_prefix("Loaded: ") {
                // Match and get rid of "Loaded: "
                if let Some(line) = line.strip_prefix("loaded ") {
                    u.state = State::Loaded;
                    let line = line.strip_prefix('(').unwrap();
                    let line = line.strip_suffix(')').unwrap();
                    let items: Vec<&str> = line.split(';').collect();
                    u.script = items[0].trim().to_string();
                    u.auto_start = match AutoStartStatus::from_str(items[1].trim()) {
                        Ok(x) => x,
                        Err(e) => {
                            println!("lol: {:?} -> {e}", items[1].trim());
                            AutoStartStatus::Disabled
                        },
                    };
                    if items.len() > 2 {
                        // preset is optionnal ?
                        u.preset = items[2].trim().ends_with("enabled");
                    }
                } else if line.starts_with("masked") {
                    u.state = State::Masked;
                }
            } else if let Some(line) = line.strip_prefix("Transient: ") {
                if line == "yes" {
                    u.transient = true
                }
            } else if line.starts_with("Active: ") {
                // skip that one
                // we already have .active() .inative() methods
                // to access this information
            } else if let Some(line) = line.strip_prefix("Docs: ") {
                is_doc = true;
                if let Ok(doc) = Doc::from_str(line) {
                    docs.push(doc)
                }
            } else if let Some(line) = line.strip_prefix("What: ") {
                // mountpoint infos
                u.mounted = Some(line.to_string())
            } else if let Some(line) = line.strip_prefix("Where: ") {
                // mountpoint infos
                u.mountpoint = Some(line.to_string());
            } else if let Some(line) = line.strip_prefix("Main PID: ") {
                // example -> Main PID: 787 (gpm)
                if let Some((pid, proc)) = line.split_once(' ') {
                    u.pid = Some(pid.parse::<u64>().unwrap_or(0));
                    u.process = Some(proc.replace(&['(', ')'][..], ""));
                };
            } else if let Some(line) = line.strip_prefix("Cntrl PID: ") {
                // example -> Main PID: 787 (gpm)
                if let Some((pid, proc)) = line.split_once(' ') {
                    u.pid = Some(pid.parse::<u64>().unwrap_or(0));
                    u.process = Some(proc.replace(&['(', ')'][..], ""));
                };
            } else if line.starts_with("Process: ") {
                //TODO: implement
                //TODO: parse as a Process item
                //let items : Vec<_> = line.split_ascii_whitespace().collect();
                //let proc_pid = u64::from_str_radix(items[1].trim(), 10).unwrap();
                //let cli;
                //Process: 640 ExecStartPre=/usr/sbin/sshd -t (code=exited, status=0/SUCCESS)
            } else if line.starts_with("CGroup: ") {
                //TODO: implement
                //LINE: "CGroup: /system.slice/sshd.service"
                //LINE: "└─1050 /usr/sbin/sshd -D"
            } else if line.starts_with("Tasks: ") {
                //TODO: implement
            } else if let Some(line) = line.strip_prefix("Memory: ") {
                u.memory = Some(line.trim().to_string());
            } else if let Some(line) = line.strip_prefix("CPU: ") {
                u.cpu = Some(line.trim().to_string())
            } else {
                // handling multi line cases
                if is_doc {
                    let line = line.trim_start();
                    if let Ok(doc) = Doc::from_str(line) {
                        docs.push(doc)
                    }
                }
            }
        }

        if let Ok(content) = cat(name) {
            let line_tuple = content
                .lines()
                .filter_map(|line| line.split_once('=').to_owned());
            for (k, v) in line_tuple {
                let val = v.to_string();
                match k {
                    "Wants" => u.wants.get_or_insert_with(Vec::new).push(val),
                    "WantedBy" => u.wanted_by.get_or_insert_with(Vec::new).push(val),
                    "Also" => u.also.get_or_insert_with(Vec::new).push(val),
                    "Before" => u.before.get_or_insert_with(Vec::new).push(val),
                    "After" => u.after.get_or_insert_with(Vec::new).push(val),
                    "ExecStart" => u.exec_start = Some(val),
                    "ExecReload" => u.exec_reload = Some(val),
                    "Restart" => u.restart_policy = Some(val),
                    "KillMode" => u.kill_mode = Some(val),
                    _ => {},
                }
                // }
            }
        }

        u.active = is_active(name)?;
        u.name = name.to_string();
        Ok(u)
    }

    /// Restarts Self by invoking `systemctl`
    pub fn restart(&self) -> std::io::Result<ExitStatus> {
        restart(&self.name)
    }

    /// Returns verbose status for Self
    pub fn status(&self) -> std::io::Result<String> {
        status(&self.name)
    }

    /// Returns `true` if Self is actively running
    pub fn is_active(&self) -> std::io::Result<bool> {
        is_active(&self.name)
    }

    /// `Isolate` Self, meaning stops all other units but
    /// self and its dependencies
    pub fn isolate(&self) -> std::io::Result<ExitStatus> {
        isolate(&self.name)
    }

    /// `Freezes` Self, halts self and CPU load will
    /// no longer be dedicated to its execution.
    /// This operation might not be feasible.
    /// `unfreeze()` is the mirror operation
    pub fn freeze(&self) -> std::io::Result<ExitStatus> {
        freeze(&self.name)
    }

    /// `Unfreezes` Self, exists halted state.
    /// This operation might not be feasible.
    pub fn unfreeze(&self) -> std::io::Result<ExitStatus> {
        unfreeze(&self.name)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_status() {
        let status = status("sshd");
        assert!(status.is_ok());
        println!("sshd status : {:#?}", status)
    }
    #[test]
    fn test_is_active() {
        let units = vec!["sshd", "dropbear", "ntpd"];
        for u in units {
            let active = is_active(u);
            assert!(active.is_ok());
            println!("{} is-active: {:#?}", u, active);
        }
    }
    #[test]
    fn test_service_exists() {
        let units = vec![
            "sshd",
            "dropbear",
            "ntpd",
            "example",
            "non-existing",
            "dummy",
        ];
        for u in units {
            let ex = exists(u);
            assert!(ex.is_ok());
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
    fn test_non_existing_unit() {
        let unit = Unit::from_systemctl("non-existing");
        assert_eq!(unit.is_err(), true);
    }
    #[test]
    fn test_service_unit_construction() {
        let units = list_units(None, None, None).unwrap(); // all units
        for unit in units {
            let unit = unit.as_str();
            if unit.contains("@") {
                // not testing this one
                // would require @x service # identification / enumeration
                continue;
            }
            println!("{unit}");
            let c0 = unit.chars().nth(0).unwrap();
            if c0.is_alphanumeric() {
                // valid unit name --> run test
                let u = match Unit::from_systemctl(&unit) {
                    Ok(x) => x,
                    Err(e) => {
                        println!("Could not parse {unit} -> {e}");
                        continue;
                    },
                };
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
    #[test]
    fn test_unit_list() {
        let units = list_units_full(None, None, None).unwrap(); // all units
        for unit in units {
            println!("####################################");
            println!("Unit: {}", unit.unit_file);
            println!("State: {}", unit.state);
            println!("Vendor Preset: {:?}", unit.vendor_preset);
            println!("####################################");
        }
    }
}
