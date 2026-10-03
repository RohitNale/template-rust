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

---

## 2. Fullstack Frameworks

Selecting the "best" framework for full-stack Rust development in 2026 depends heavily on your performance needs, architectural preferences, and experience with WebAssembly (WASM). There is no single winner, but rather a set of specialized tools for different project goals.
Leading Full-Stack Frameworks

- Leptos: Often considered the industry standard for performance-oriented full-stack Rust. It uses fine-grained reactivity and compiles to WASM, making it excellent for apps requiring high-speed UI interaction. It is a mature, production-tested ecosystem.
- Dioxus: A versatile choice that feels similar to React. Its standout feature is its cross-platform capability; you can use the same codebase to build web, desktop, and mobile apps. It is widely favored by teams that need to deploy beyond just the browser.
- Topcoat: A newer, "batteries-included" framework from the Tokio team. Unlike its predecessors, it prioritizes simplicity and server-side rendering (SSR). Instead of using WASM for the entire client, it uses a thin JavaScript layer, which can result in faster initial page loads and a more approachable learning curve for those coming from traditional frameworks like Ruby on Rails or Laravel.

How to Choose

- Choose Leptos if: You need maximum performance, complex client-side logic, and want a mature, battle-tested ecosystem that leverages the full power of Rust in the browser via WASM.
- Choose Dioxus if: You value code reuse across different platforms (web, desktop, mobile) and prefer a developer experience similar to React.
- Choose Topcoat if: You are building a traditional multi-page application, prefer a simpler "server-first" model, or want to avoid the complexities and bundle sizes associated with full WASM frontends.

For pure backend development, Axum remains the gold standard for high-performance APIs and is frequently used as the foundation for these full-stack frameworks.