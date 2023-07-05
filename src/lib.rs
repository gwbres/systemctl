//! Crate to manage and monitor services through `systemctl`   
//! Homepage: <https://github.com/gwbres/systemctl>
use std::io::{Error, ErrorKind, Read};
use std::process::ExitStatus;
use std::str::FromStr;
use strum_macros::EnumString;



/// Invokes `systemctl $args` silently
fn _systemctl(args: Vec<&str>) -> std::io::Result<ExitStatus> {
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
    //TODO improve this please
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
pub fn _restart(unit: &str) -> std::io::Result<ExitStatus> {
    _systemctl(vec!["restart", unit])
}

/// Forces given `unit` to stop
pub fn _stop(unit: &str) -> std::io::Result<ExitStatus> {
   _systemctl(vec!["stop", unit])
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
pub fn _isolate(unit: &str) -> std::io::Result<ExitStatus> {
    _systemctl(vec!["isolate", unit])
}

/// Freezes (halts) given unit.
/// This operation might not be feasible.
pub fn _freeze(unit: &str) -> std::io::Result<ExitStatus> {
   _systemctl(vec!["freeze", unit])
}

/// Unfreezes given unit (recover from halted state).
/// This operation might not be feasible.
pub fn _unfreeze(unit: &str) -> std::io::Result<ExitStatus> {
    _systemctl(vec!["thaw", unit])
}

/// Returns `true` if given `unit` exists,
/// ie., service could be or is actively deployed
/// and manageable by systemd
pub fn exists(unit: &str) -> std::io::Result<bool> {
    let status = status(unit);
    Ok(status.is_ok()
        && !status
            .unwrap()
            .trim_end()
            .eq(&format!("Unit {}.service could not be found.", unit)))
}

/// Returns list of units extracted from systemctl listing.   
///  + type filter: optionnal --type filter
///  + state filter: optionnal --state filter
pub fn list_units(
    type_filter: Option<&str>,
    state_filter: Option<&str>,
) -> std::io::Result<Vec<String>> {
    let mut args = vec!["list-unit-files"];
    if let Some(filter) = type_filter {
        args.push("--type");
        args.push(filter)
    }
    if let Some(filter) = state_filter {
        args.push("--state");
        args.push(filter)
    }
    let mut result: Vec<String> = Vec::new();
    let content = systemctl_capture(args)?;
    let lines = content.lines();
    for l in lines.skip(1) {
        // header labels
        let parsed: Vec<_> = l.split_ascii_whitespace().collect();
        if parsed.len() >= 2 {
            result.push(parsed[0].to_string())
        }
    }
    Ok(result)
}

/// Returns list of services that are currently declared as disabled
pub fn _list_disabled_services() -> std::io::Result<Vec<String>> {
    Ok(list_units(Some("service"), Some("disabled"))?)
}

/// Returns list of services that are currently declared as enabled
pub fn _list_enabled_services() -> std::io::Result<Vec<String>> {
    Ok(list_units(Some("service"), Some("enabled"))?)
}

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
    #[strum(serialize = "transient")]
    Transient,
    #[strum(serialize = "enabled-runtime")]
    EnabledRuntime,

}

impl ToString for AutoStartStatus{
    fn to_string(&self) -> String {
        match self {
            AutoStartStatus::Static => "static".to_string(),
            AutoStartStatus::Enabled => "enabled".to_string(),
            AutoStartStatus::Disabled => "disabled".to_string(),
            AutoStartStatus::Generated => "generated".to_string(),
            AutoStartStatus::Indirect => "indirect".to_string(),
            AutoStartStatus::Transient => "transient".to_string(),
            AutoStartStatus::EnabledRuntime => "enabled-runtime".to_string(),
        }
    }
}

impl Default for AutoStartStatus {
    fn default() -> AutoStartStatus {
        AutoStartStatus::Disabled
    }
      
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
    #[strum(serialize = "swap")]
    Swap,
    #[strum(serialize = "aa-prompt-listener")]
    AaPromptListener,
    #[strum(serialize = "system-shutdown")]
    SystemShutdown,
    #[strum(serialize = "recovery-chooser-trigger")]
    RecoveryChooserTrigger,
    #[strum(serialize = "failure")]
    Failure,
    #[strum(serialize = "unmount")]
    Unmount,
    #[strum(serialize = "autoimport")]
    AutoImport,
    #[strum(serialize = "snap-repair")]
    SnapRepair,
    #[strum(serialize = "mounts-pre")]
    MountsPre,
    #[strum(serialize = "mounts-post")]
    MountsPost,
    #[strum(serialize = "mounts")]
    Mounts,
    #[strum(serialize = "seeded")]
    Seeded,
    #[strum(serialize = "apparmor")]
    Apparmor,
    #[strum(serialize = "core-fixup")]
    CoreFixup,
}

impl Default for Type {
    fn default() -> Type {
        Type::Service
    }
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
    fn default() -> State {
        State::Masked
    }
}



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
    pub fn _as_man(&self) -> Option<&str> {
        match self {
            Doc::Man(s) => Some(&s),
            _ => None,
        }
    }
    /// Unwrapps self as webpage `Url`
    pub fn _as_url(&self) -> Option<&str> {
        match self {
            Doc::Url(s) => Some(&s),
            _ => None,
        }
    }
}

impl std::str::FromStr for Doc {
    type Err = std::io::Error;
    /// Builds `Doc` from systemd status descriptor
    fn from_str(status: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = status.split(":").collect();
        if items.len() != 2 {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "malformed doc descriptor",
            ));
        }
        match items[0] {
            "man" => {
                let content: Vec<&str> = items[1].split("(").collect();
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
            cpu: Default::default(),
            memory: Default::default(),
            state: Default::default(),
            auto_start: Default::default(),
            preset: Default::default(),
            active: Default::default(),
            docs: Default::default(),
            process: Default::default(),
            mounted: Default::default(),
            mountpoint: Default::default(),
            wants: Default::default(),
            wanted_by: Default::default(),
            restart_policy: Default::default(),
            kill_mode: Default::default(),
            after: Default::default(),
            before: Default::default(),
            also: Default::default(),
            exec_start: Default::default(),
            exec_reload: Default::default(),
        }
    }
}

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
        let status = status(name)?;
        let mut lines = status.lines();
        let next = lines.next().unwrap();
        let (_, rem) = next.split_at(3);
        let mut items = rem.split_ascii_whitespace();
        let name = items.next().unwrap().trim();
        let mut description: Option<String> = None;
        if let Some(delim) = items.next() {
            if delim.trim().eq("-") {
                // --> description string is provided
                let items: Vec<_> = items.collect();
                description = Some(itertools::join(&items, " "));
            }
        }
       
        let items: Vec<_> = name.split_terminator(".").collect();
        // `type` is deduced from .extension
        let utype = Type::from_str(items[1].trim()).unwrap();
        let mut script: String = String::new();

        let mut pid: Option<u64> = None;
        let mut process: Option<String> = None;

        let mut state: State = State::default();
        let mut auto_start: AutoStartStatus = AutoStartStatus::default();

        let mut preset: bool = false;
        let mut cpu: Option<String> = None;
        let mut memory: Option<String> = None;
        let mut mounted: Option<String> = None;
        let mut mountpoint: Option<String> = None;

        let mut docs: Vec<Doc> = Vec::with_capacity(3);
        let mut is_doc: bool = false;

        let mut wants: Vec<String> = Vec::new();
        let mut wanted_by: Vec<String> = Vec::new();
        let mut before: Vec<String> = Vec::new();
        let mut after: Vec<String> = Vec::new();
        let mut also: Vec<String> = Vec::new();
        let mut exec_start = String::new();
        let mut exec_reload = String::new();
        let mut kill_mode = String::new();
        let mut restart_policy = String::new();
        
        for line in lines {
            let line = line.trim_start();
            if line.starts_with("Loaded:") {
                let (_, line) = line.split_at(8); // Get rid of "Loaded: "
                if line.starts_with("loaded") {
                    state = State::Loaded;
                    let (_, rem) = line.split_at(1); // remove "("
                    let (rem, _) = rem.split_at(rem.len() - 1); // remove ")"
                    let items: Vec<_> = rem.split_terminator(";").collect();
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
                // skip that one
                // we already have .active() .inative() methods
                // to access this information
            } else if line.starts_with("Docs: ") {
                is_doc = true;
                let (_, line) = line.split_at(6); // remove "Docs: "
                if let Ok(doc) = Doc::from_str(line) {
                    docs.push(doc)
                }
            } else if line.starts_with("What: ") {
                // mountpoint infos
                mounted = Some(line.split_at(6).1.trim().to_string());
            } else if line.starts_with("Where: ") {
                // mountpoint infos
                mountpoint = Some(line.split_at(7).1.trim().to_string());
            } else if line.starts_with("Main PID: ") {
                // Main PID: 787 (gpm)
                let items: Vec<&str> = line.split_ascii_whitespace().collect();
                pid = Some(u64::from_str_radix(items[2].trim(), 10).unwrap());
                process = Some(items[3].replace(")", "").replace("(", "").to_string())
            } else if line.starts_with("Process: ") {
                //TODO: parse as a Process item
                //let items : Vec<_> = line.split_ascii_whitespace().collect();
                //let proc_pid = u64::from_str_radix(items[1].trim(), 10).unwrap();
                //let cli;
                //Process: 640 ExecStartPre=/usr/sbin/sshd -t (code=exited, status=0/SUCCESS)
            } else if line.starts_with("CGroup: ") {
                //LINE: "CGroup: /system.slice/sshd.service"
                //LINE: "└─1050 /usr/sbin/sshd -D"
            } else if line.starts_with("Tasks: ") {
            } else if line.starts_with("Memory: ") {
                let line = line.split_at(8).1;
                memory = Some(line.trim().to_string())
            } else if line.starts_with("CPU: ") {
                let line = line.split_at(5).1;
                cpu = Some(line.trim().to_string())
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
            let lines = content.lines();
            for line in lines {
                if line.contains("=") {
                    let items: Vec<&str> = line.split("=").collect();
                    let key = items[0];
                    let value = items[1].trim();
                    // println!("Key {} Value {}", key, value);
                    match key {
                        "Wants" => wants.push(value.to_string()),
                        "WantedBy" => wanted_by.push(value.to_string()),
                        "Also" => also.push(value.to_string()),
                        "Before" => before.push(value.to_string()),
                        "After" => after.push(value.to_string()),
                        "ExecStart" => exec_start = value.to_string(),
                        "ExecReload" => exec_reload = value.to_string(),
                        "Restart" => restart_policy = value.to_string(),
                        "KillMode" => kill_mode = value.to_string(),
                        _ => {},
                    }
                }
            }
        }

        Ok(Unit {
            name: name.to_string(),
            description,
            script,
            utype,
            process,
            pid,
            state,
            auto_start,
            restart_policy: {
                if restart_policy.len() > 0 {
                    Some(restart_policy)
                } else {
                    None
                }
            },
            kill_mode: {
                if kill_mode.len() > 0 {
                    Some(kill_mode)
                } else {
                    None
                }
            },
            preset,
            active: is_active(name)?,
            tasks: Default::default(),
            cpu,
            memory,
            mounted,
            mountpoint,
            docs: {
                if docs.len() > 0 {
                    Some(docs)
                } else {
                    None
                }
            },
            wants: {
                if wants.len() > 0 {
                    Some(wants)
                } else {
                    None
                }
            },
            wanted_by: {
                if wanted_by.len() > 0 {
                    Some(wanted_by)
                } else {
                    None
                }
            },
            before: {
                if before.len() > 0 {
                    Some(before)
                } else {
                    None
                }
            },
            also: {
                if also.len() > 0 {
                    Some(also)
                } else {
                    None
                }
            },
            after: {
                if after.len() > 0 {
                    Some(after)
                } else {
                    None
                }
            },
            exec_start: {
                if exec_start.len() > 0 {
                    Some(exec_start)
                } else {
                    None
                }
            },
            exec_reload: {
                if exec_reload.len() > 0 {
                    Some(exec_reload)
                } else {
                    None
                }
            },
        })
    }

    /// Restarts Self by invoking `systemctl`
    pub fn _restart(&self) -> std::io::Result<ExitStatus> {
        _restart(&self.name)
    }

    /// Returns verbose status for Self
    pub fn status(&self) -> std::io::Result<String> {
        status(&self.name)
    }

    /// Returns `true` if Self is actively running
    pub fn _is_active(&self) -> std::io::Result<bool> {
        is_active(&self.name)
    }

    /// `Isolate` Self, meaning stops all other units but
    /// self and its dependencies
    pub fn _isolate(&self) -> std::io::Result<ExitStatus> {
        _isolate(&self.name)
    }

    /// `Freezes` Self, halts self and CPU load will
    /// no longer be dedicated to its execution.
    /// This operation might not be feasible.
    /// `unfreeze()` is the mirror operation
    pub fn _freeze(&self) -> std::io::Result<ExitStatus> {
        _freeze(&self.name)
    }

    /// `Unfreezes` Self, exists halted state.
    /// This operation might not be feasible.
    pub fn _unfreeze(&self) -> std::io::Result<ExitStatus> {
        _unfreeze(&self.name)
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
        let units = vec!["sshd", "dropbear", "ntpd"];
        for u in units {
            let active = is_active(u);
            assert_eq!(active.is_ok(), true);
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
            assert_eq!(ex.is_ok(), true);
            println!("{} exists: {:#?}", u, ex);
        }
    }
    #[test]
    fn test_disabled_services() {
        let services = _list_disabled_services().unwrap();
        println!("disabled services: {:#?}", services)
    }
    #[test]
    fn test_enabled_services() {
        let services = _list_enabled_services().unwrap();
        println!("enabled services: {:#?}", services)
    }
    #[test]
    fn test_non_existing_unit() {
        let unit = Unit::from_systemctl("non-existing");
        assert_eq!(unit.is_err(), true);
    }

    //Auxiliar function for next test
    fn contains_numbers(s: &str) -> bool {
        for c in s.chars() {
            if c.is_numeric() {
                return true;
            }
        }
        false
    }

    #[test]
    fn test_service_unit_construction() {
        let units = list_units(None, None).unwrap(); // all units
        assert_eq!(units.len() > 0, true);
        for unit in units {
            let unit = unit.as_str();
            if unit.contains("@") {
                // not testing this one
                // would require @x service # identification / enumeration
                continue;
            }
            if contains_numbers(&unit) {
                //if you try to unwrap a unit with a name containing numbers it will give out an error
                //for now this is a quick fix to avoid that
                //this problem needs to be looked in to in detail
                continue;
            }
            
            let c0 = unit.chars().nth(0).unwrap();
            if c0.is_alphanumeric() {
                // valid unit name --> run test
                println!("Unit: {:?}", &unit);
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
