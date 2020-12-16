use std::io;
use std::io::prelude::*;

mod get_dotnet_version;

fn main() {
    println!("- check 4.5 later version.");
    get_dotnet_version::get_45_plus_from_registry();
    println!();

    println!("- check 1-4 version.");
    get_dotnet_version::get_version_from_registry();
    println!();

    let mut _buf = String::new();
    let _ = io::stdin().lock().read_line(&mut _buf);
}
