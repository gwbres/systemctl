//! systemctl: small crate to interact with services through systemctl
//! Homepage: <https://github.com/gwbres/systemctl>
//use thiserror::Error;
use std::process::ExitStatus;
use std::io::{Read, Error, ErrorKind};
/*
// Structure to describe a systemd `unit`
//struct Unit {
    /// Service script loaded when starting this unit
    //service_script: &str, 

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

/// Forces given `unit` (re)start
pub fn restart (unit: &str) -> std::io::Result<ExitStatus> {
    let mut child = std::process::Command::new("/usr/bin/systemctl")
        .arg("restart")
        .arg(unit)
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    child.wait()
}

/// Forces given `unit` to stop
pub fn stop (daemon: &str) -> std::io::Result<ExitStatus> {
    let mut child = std::process::Command::new("/usr/bin/systemctl")
        .arg("stop")
        .arg(daemon)
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    child.wait()
}

/// Returns `true` if given `unit` is actively running
pub fn is_active (unit: &str) -> std::io::Result<bool> {
    let mut child = std::process::Command::new("/usr/bin/systemctl")
        .arg("is-active")
        .arg(unit)
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let exitcode = child.wait()?;
    match exitcode.success() {
        true => {
            let mut stdout : Vec<u8> = Vec::new();
            if let Ok(size) = child.stdout.unwrap().read_to_end(&mut stdout) {
                if size > 0 {
                    if let Ok(s) = String::from_utf8(stdout) {
                        Ok(s.contains("Active: active (running)"))
                    } else {
                        Err(Error::new(ErrorKind::InvalidData, "Invalid utf8 data"))
                    }
                } else {
                    Ok(false)
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

// list all disabled units
//systemctl list-unit-files --type service --state disabled
