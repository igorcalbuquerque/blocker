use crate::hosts_manager::HostManager;
use std::collection::HashSet;

pub struct Blocker {
    manager: HostManager,
    blocked_sites: HashSet<String>,
}

impl Blocker {
    pub fn new() -> Self {
        let manager = HostManager::new();
        let blocked_sites = HashSet::new();

        Blocker { manager, blocked_sites }
    }

    pub fn load_blocked_sites(&mut self) {
        if let Ok(content) = self.manager.read_hosts() {
            for line in content.lines() {
                if line.contains("# Blocker") {
                    if let Some(site) = line.split_whitespace().nth(1) {
                        self.blocked_sites.insert(site.to_string());
                    }
                }
            }
        }
    }

    pub fn add_site(&mut self, site: &str) {
        self.blocked_sites.insert(site.to_string());
    }

    pub fn remove_site(&mut self, site: &str) {
        self.blocked_sites.remove(site);
    }

    pub fn apply_changes(&self) {
        let mut content = String::new();

        if let Ok(existing_content) = self.manager.read_hosts() {
            for line in existing_content.lines() {
                if !line.contains("# Blocker") {
                    content.push_str(line);
                    content.push('\n');
                }
            }
        }

        for site in &self.blocked_sites {
            content.push_str(&format!("127.0.0.1\t{}\t# Blocker\n", site));
        }

        if let Err(e) = self.manager.write_hosts(&content) {
            eprintln!("Error for write hosts file: {}", e);
        }
    }

    pub fn list_blocked_sites(&self) -> Vec<String> {
        self.blocked_sites.iter().cloned().collect()
    }
}