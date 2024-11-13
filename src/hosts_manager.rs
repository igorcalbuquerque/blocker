use std::fs::{OpenOptions, read_to_string};
use std::io::{Write, Result};
use std::path::Path;

pub struct HostManager {
    path: String,
}

impl HostManager {
    pub fn new() -> Self {
        let path = if cftg!(target_os = "windows") {
            r"C:\Windows\System32\drivers\etc\hosts".to_string()
        } else {
            "etc/local/hosts".to_string()
        };

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

}