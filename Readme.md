# Blocker

Blocker is a command-line application written in Rust that allows you to block access to specified websites on your computer. By modifying the hosts file, Blocker redirects domains to localhost, effectively preventing access to the listed sites. This tool can help increase productivity by blocking distracting websites or enhance parental controls by restricting access to inappropriate content.

## Features

- **Add Websites to Block**: Easily specify which websites you want to block.
- **Remove Blocked Websites**: Remove websites from the block list at any time.
- **List Blocked Websites**: View all currently blocked websites.
- **Apply Changes**: Updates take effect immediately after applying changes.
- **Cross-Platform Support**: Works on Windows, macOS, Linux, and WSL.

## Requirements

- **Rust**: Ensure you have Rust installed. Download it from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Administrator Privileges**: Modifying the hosts file requires admin/root access.
- **Dependencies**:
  - **Unix/Linux/macOS**: Uses the `nix` crate.
  - **Windows**: Uses the `winapi` crate.
  - Dependencies are managed via Cargo features.

## Build the Application
- For Unix/Linux/macOS:
```bash
cargo build --features unix
```
- For Windows:
```bash
cargo build --features windows
```
