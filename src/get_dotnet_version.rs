use std::io;
use winreg::enums::*;
use winreg::RegKey;

pub fn get_45_plus_from_registry() {
    let npd_sub_key = r#"SOFTWARE\Microsoft\NET Framework Setup\NDP\v4\Full\"#;

    let ndp_key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(npd_sub_key)
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
        // This code should never execute. A non-null release key should mean
        // that 4.5 or later is installed.
        _ => "No 4.5 or later version detected",
    };
    println!(".NET Framework Version: {}", version);
}

pub fn get_version_from_registry() {
    let npd_sub_key = r#"SOFTWARE\Microsoft\NET Framework Setup\NDP\"#;

    let ndp_key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(npd_sub_key)
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                panic!("No .NET Framework installed.")
            }
            io::ErrorKind::PermissionDenied => panic!("Access denied"),
            _ => panic!("{:?}", e),
        });

    for version_key_name in ndp_key.enum_keys().map(|k| k.unwrap()) {
        // Skip .NET Framework 4.5 version information.
        if "v4" == version_key_name {
            continue;
        }

        if version_key_name.starts_with("v") {
            if let Ok(version_key) = ndp_key.open_subkey(&version_key_name) {
                // Get the .NET Framework version value.
                let name_key: io::Result<String> = version_key.get_value("Version");
                // Get the service pack (SP) number.
                let sp_key: io::Result<u32> = version_key.get_value("SP");

                // Get the installation flag, or an empty string if there is none.
                let install_key: io::Result<u32> = version_key.get_value("Install");

                if let Ok(_) = install_key {
                    if let (Ok(1), Ok(sp)) = (install_key.as_ref(), sp_key.as_ref()) {
                        println!(
                            "{0}  {1}  SP{2}",
                            &version_key_name,
                            name_key.as_ref().unwrap(),
                            sp
                        );
                    }
                } else {
                    println!(
                        "{0}  {1}",
                        &version_key_name,
                        name_key.as_ref().unwrap_or(&String::from(""))
                    );
                }

                if let Ok(_) = name_key {
                    continue;
                }

                for sub_key_name in version_key.enum_keys().map(|k| k.unwrap()) {
                    let sub_key = version_key.open_subkey(&sub_key_name).unwrap();
                    let name_key: io::Result<String> = sub_key.get_value("Version");
                    let sp_key: io::Result<u32> = sub_key.get_value("SP");
                    let install_key: io::Result<u32> = sub_key.get_value("Install");

                    if let Ok(_) = install_key {
                        if let (Ok(1), Ok(sp)) = (install_key.as_ref(), sp_key.as_ref()) {
                            println!(
                                "{0}  {1}  SP{2}",
                                &sub_key_name,
                                name_key.as_ref().unwrap_or(&String::from("")),
                                sp
                            );
                        } else if let Ok(1) = install_key {
                            println!(
                                "  {0}  {1}",
                                &sub_key_name,
                                name_key.as_ref().unwrap_or(&String::from(""))
                            );
                        }
                    } else {
                        println!(
                            "{0}  {1}",
                            &version_key_name,
                            name_key.as_ref().unwrap_or(&String::from(""))
                        );
                    }
                }
            }
        }
    }
}
