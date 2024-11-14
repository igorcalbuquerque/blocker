mod hosts_manager;
mod blocker;

use blocker::Blocker;
use std::io::{self, Write};

fn main() {

    if !am_i_root() {
        eprint!("ERROR! admins only");
        return;
    }

    let mut blocker = Blocker::new();
    blocker.load_blocked_sites();

    loop {
        println!("========= Blocker =========");
        println!("1 - Add site for block");
        println!("2 - Remove site from block list");
        println!("3 - List blocked sites");
        println!("4 - Apply Changes");
        println!("0 - Exit");
        println!("Option: ");

        io::stdout().flush().unwrap();

        let mut opt = String::new();
        io::stdin().read_line(&mut opt).unwrap();

        match opt.trim() {
            "1" => {
                print!("Enter the Domain to block (e.g., www.example.com)");
                io::stdout().flush().unwrap();
                let mut site = String::new();
                io::stdin().read_line(&mut site).unwrap();
                blocker.add_site(site.trim());
                println!("Site added!");
            },
            "2" => {
                print!("Enter the Domain to remove: ");
                io::stdout().flush().unwrap();
                let mut site = String::new();
                io::stdin().read_line(&mut site).unwrap();
                blocker.remove_site(site.trim());
                println!("Site removed from block list!");
            },
            "3" => {
                println!("Blocked sites: ");
                for site in blocker.list_blocked_sites() {
                    println!("- {}", site);
                }
            },
            "4" => {
                blocker.apply_changes();
                println!("Changes applied successfully");
            },
            "0" => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Invalid option. Try again");
            }            
        }

    }

}

#[cfg(feature = "unix")]
fn am_i_root() -> bool {
    // For Unix/Linux/macOS systems
    use nix::unistd::Uid;
    Uid::effective().is_root()
}
#[cfg(target_family = "windows")]
fn am_i_root() -> bool {
    use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::winnt::{TokenElevation, HANDLE, TOKEN_ELEVATION, TOKEN_QUERY};

    unsafe {
        let mut token_handle: HANDLE = std::ptr::null_mut();
        let process_handle = GetCurrentProcess();
        if OpenProcessToken(process_handle, TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length = 0;

        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        );

        if result == 0 {
            false
        } else {
            elevation.TokenIsElevated != 0
        }
    }
}