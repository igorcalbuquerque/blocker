use std::fs::{OpenOptions, read_to_string};
use std::io::{Write, Result};
use std::process::Command;

pub struct HostManager {
    path: String,
}

impl HostManager {
    pub fn new() -> Self {
        let path = Self::get_hosts_path();
        HostManager { path }
    }

    /// read host file content
    pub fn read_hosts(&self) -> Result<String> {
        read_to_string(&self.path)
    }

    /// write a new content on hosts
    pub fn write_hosts(&self, content: &str) -> Result<()> {
        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.path)?;

        file.write_all(content.as_bytes())
    }

    fn get_hosts_path() -> String {
        if Self::is_wsl() {
            "/mnt/c/Windows/System32/drivers/etc/hosts".to_string()
        } else if cfg!(target_os = "windows") {
            r"C:\Windows\System32\drivers\etc\hosts".to_string()
        } else {
            "/etc/hosts".to_string()
        }
    }

    fn is_wsl() -> bool {
        if cfg!(target_os = "linux") {
            if let Ok(output) = Command::new("uname").arg("-a").output() {
                let uname_output = String::from_utf8_lossy(&output.stdout);
                return uname_output.contains("Microsoft");
            }
        }
        false
    }

}