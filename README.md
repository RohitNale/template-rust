# Template Rust Project

Follow the steps below based on your operating system to install Rust using **rustup** (the official toolchain installer) and check its version.

---

## 1. Installation

### 🍏 macOS & 🐧 Linux
Open your terminal and run the following command to download and run the official installation script:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

*When prompted during installation, press `1` and hit **Enter** to proceed with the default setup.*

### 🪟 Windows
1. Download the **rustup-init.exe** installer from the official [Rust Downloads](https://rust-lang.org "Rust Installation Page") page.
2. Run the `.exe` file.
3. If prompted, install the required Microsoft C++ Build Tools.
4. Press `1` and hit **Enter** to proceed with the default installation.

---

## 2. Configure Environment Path (If Required)
After installation, you may need to restart your terminal window or manually source the environment variables for the changes to take effect.

* **Linux/macOS:** Run `source $HOME/.cargo/env` or restart your shell.
* **Windows:** Close your current command prompt/PowerShell window and open a new one.

---

## 3. Check Installed Versions

To verify that the Rust compiler (`rustc`) and its package manager (`cargo`) were successfully installed, run the following commands:

### Check Rust Compiler Version
```bash
rustc --version
```
*Expected Output Format:* `rustc x.y.z (abcabcabc yyyy-mm-dd)`

### Check Cargo (Package Manager) Version
```bash
cargo --version
```

### Check Rustup Version
```bash
rustup --version
```

---

## 4. Keeping Rust Up to Date
To update your installation to the latest stable release in the future, run:

```bash
rustup update
```
