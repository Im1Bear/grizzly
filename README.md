# 🐻 Grizzly

Grizzly is a fast, minimal Windows system cleaner written in Rust. It eats your junk files so you don't have to.

---

## Installation

### Build from source

Make sure you have [Rust](https://rustup.rs/) installed, then:

```bash
git clone https://github.com/im1bear/grizzly
cd grizzly
cargo build --release
```

The binary will be at `target/release/grizzly.exe`.

---

## Usage

> **Grizzly requires administrator privileges.** Run your terminal as Administrator before using it.

```bash
grizzly eat
```

or

```bash
grizzly clean
```

Both commands are equivalent — Grizzly will clean all supported locations and display a summary when done.

---

## What Grizzly cleans

| Category | Location |
|---|---|
| Temp files | `%TEMP%`, `C:\Windows\Temp` |
| Browser cache | Chrome, Firefox, Edge |
| Prefetch | `C:\Windows\Prefetch` |
| Event Logs | Application, System, Security |
| Recycle Bin | All drives |

---

## Example output

```
Übersprungen (Datei blockiert): "example.tmp" - os error 32
Gelöscht (Datei): "catalog.json"
Gelöscht (Ordner): "Cache_Data"
...

🐻 Grizzly hat gefressen:
├── Elemente: 28
└── Größe:    17.66 MB
```

Blocked files (in use by other processes) are skipped automatically — Grizzly never forces deletion of locked files.

---

## Admin rights

Grizzly requests administrator privileges automatically via a Windows manifest. If you run it without elevated rights, Windows will prompt you or the process will be blocked.

---

## Built with

- [Rust](https://www.rust-lang.org/)
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [windows-rs](https://github.com/microsoft/windows-rs) — Windows API bindings

---

*Grizzly eats junk. That's what bears do.* 🐻