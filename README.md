# ⚙️ DevOps Toolbox in Rust

A collection of minimal, fast, and CI/CD-ready DevOps tools written in Rust.

This project showcases how Rust can be used to build production-grade system utilities with clear structure, safe execution, and excellent performance — all designed for Linux environments and terminal-based operations.

---

## 🧱 Modules

| Tool        | Description                                | Status     |
|-------------|--------------------------------------------|------------|
| `netcheck`  | HTTP health checker with logging + JSON    | ✅ Complete |
| `procwatch` | Process monitor with CLI filtering         | ✅ Complete |
| `perfstat`  | *(Coming soon)* CPU, memory, disk usage    | 🕓 Planned  |
| `netpeek`   | *(Coming soon)* Networking probes + DNS    | 🕓 Planned  |
| `textshaper`| *(Coming soon)* Text/Log filtering toolkit | 🕓 Planned  |

---

## 📦 Folder Structure

```

rust-devops/
├── netcheck/       # HTTP check CLI
├── procwatch/      # Process monitoring CLI
├── perfstat/       # (to be added)
├── netpeek/        # (to be added)
├── textshaper/     # (to be added)
└── README.md       # This file

````

---

## 🌐 Module: `netcheck`

**A minimal Rust CLI for HTTP health checks.**  
Built for DevOps pipelines, CI/CD integration, and infrastructure observability.

### 🚀 Features

- Takes a URL as input
- Performs a `GET` request
- Measures response time
- Logs output to file and JSON
- Exits with status `0` (success) or `1` (failure)

### 🎥 Demo

![Demo](netcheck/.assets/demo1.gif)

### 🧰 Usage

```bash
cd netcheck
make build
make run
./target/release/netcheck https://example.com
````

### 📁 Output

* JSON: `output/netcheck.json`
* Logs: `logs/netcheck.log`

---

## 🧠 Module: `procwatch`

**A lightweight Rust CLI for Linux process monitoring.**
Built for system introspection, CI pipelines, and automation scenarios.

### 🚀 Features

* Lists all active processes via `/proc`
* Shows `PID`, `Name`, `Status`
* Filter by name: `--filter <name>`
* Saves output as structured JSON
* Logs executions with timestamps

### 🧰 Usage

```bash
cd procwatch
make build
./target/release/procwatch --filter sshd
```

### 📁 Output

* JSON: `output/procs.json`
* Logs: `logs/procwatch.log`

### 📤 Example

```
[322] sshd (Sleeping)
[1311] sshd (Sleeping)
```

```json
[
  {
    "pid": 322,
    "name": "sshd",
    "status": "Sleeping"
  }
]
```

---

## 📜 License

MIT — free to use, modify, and distribute.

> Built with 🦀 by [Mike Niner](https://github.com/mikeninerbravog)