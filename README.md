# WinRice 🍚

WinRice is a modular, template-based, zero-residue dotfile and environment manager designed specifically for Windows ricing. It brings Linux-like ricing capabilities (such as seamless integration with Komorebi, GlazeWM, Yasb, and Zebar) into a cohesive, safe, and easily manageable ecosystem.

## 🎯 Vision & Goals

- **Zero-Residue Rollbacks:** Modifying Windows settings or configuration files can be risky. WinRice ensures that every deployment is backed by an atomic snapshot, allowing you to instantly roll back your system to a clean state without leaving orphaned files or registry junk.
- **Centralized Theming (Tera):** Instead of manually editing colors in a dozen different `.json`, `.yaml`, or `.lua` files, WinRice uses a central theme state. Through the Rust-based Tera template engine, a single color change cascades across your entire system dynamically.
- **Agent-Driven Architecture:** WinRice is built from the ground up using strict Agent-Driven Development (ADD) and Test-Driven Development (TDD) methodologies, ensuring that the codebase is highly robust, heavily tested, and "Agent-Testable."

## 🚀 Current State

The project is currently in the **backend architectural phase**. The core foundational systems have been built in Rust (Tauri backend):

1. **Atomic File Manager (`src/file_manager`):**
   - Implements strict atomic file writes using `tempfile` to prevent corruption during power loss or application crashes.
   - Handles localized snapshots (`backup.rs`) and instantaneous restoration (`rollback.rs`).

2. **Template Engine (`src/template_engine`):**
   - Integrates the `tera` templating engine.
   - Safely bridges the rendering engine with the atomic file manager (`render_and_deploy`), guaranteeing that invalid templates or failed renders never corrupt an existing dotfile.

## 🛠️ Testing

Since the UI and CLI bindings are not yet implemented, you can verify the integrity of the core engine by running the automated test suite. The test suite includes deep mutation-tested scenarios (including hardlink verification for atomicity).

```bash
cd src-tauri
cargo test
```

## 🔮 Roadmap

- [x] Pre-Production Research & Agent Orchestration Loop
- [x] Core State Management & File I/O
- [x] Tera Template Engine Integration
- [ ] **Tauri IPC Bindings:** Exposing the Rust backend to the frontend.
- [ ] **React Frontend:** Building the UI for managing themes and modules.
- [ ] **Window Manager Modules:** Official templates for Komorebi and GlazeWM.
- [ ] **Status Bar Modules:** Official templates for Yasb and Zebar.

---
*Built with ❤️ and AI (TDD/ADD workflow).*
