use std::io;
use winreg::enums::*;
use winreg::RegKey;

pub fn get_45_plus_from_registry() {
    let sub_key = r#"SOFTWARE\Microsoft\NET Framework Setup\NDP\v4\Full\"#;

    let ndp_key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(sub_key)
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                panic!(".NET Framework Version 4.5 or later is not detected.")
            }
            io::ErrorKind::PermissionDenied => panic!("Access denied"),
            _ => panic!("{:?}", e),
        });

    let release_key: u32 = ndp_key
        .get_value("Release")
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                panic!(".NET Framework Version 4.5 or later is not detected.")
            }
            io::ErrorKind::PermissionDenied => panic!("Access denied"),
            _ => panic!("{:?}", e),
        });

    let version = match release_key {
        r if r >= 528040 => "4.8 or later",
        r if r >= 461808 => "4.7.2",
        r if r >= 461308 => "4.7.1",
        r if r >= 460798 => "4.7",
        r if r >= 394802 => "4.6.2",
        r if r >= 394254 => "4.6.1",
        r if r >= 393295 => "4.6",
        r if r >= 379893 => "4.5.2",
        r if r >= 378675 => "4.5.1",
        r if r >= 378389 => "4.5",
        _ => "No 4.5 or later version detected",
    };
    println!(".NET Framework Version: {}", version);
}

pub fn get_version_from_registry() {}
