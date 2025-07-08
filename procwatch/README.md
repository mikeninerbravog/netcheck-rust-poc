## 📄 `procwatch/README.md`

````markdown
# 🧠 procwatch

A minimal Rust CLI tool to monitor active Linux processes.  
Built for DevOps pipelines, system introspection, and learning Rust with `/proc`.

---

## 🚀 What It Does

- Lists active processes from `/proc`
- Outputs: `PID`, `name`, `status`
- Supports filtering by name (`--filter`)
- Saves structured JSON (`output/`) and logs (`logs/`)
- CI/CD-ready with zero dependencies outside Linux

---

## 🛠️ Usage

### 🧱 Build

```bash
make build
````

### ▶️ Run (all processes)

```bash
make run
```

### 🔎 Run with filter

```bash
./target/release/procwatch --filter ssh
```

---

## 📂 Project Layout

```
procwatch/
├── src/main.rs           # Rust source
├── Makefile              # Build/run helper
├── input/                # Reserved for future configs
├── output/procs.json     # Result in JSON
├── logs/procwatch.log    # Timestamped execution logs
├── Cargo.toml            # Dependencies
└── README.md             # This file
```

---

## 📈 Example Output

### Terminal

```text
[322] sshd (Sleeping)
[1311] sshd (Sleeping)
```

### JSON

```json
[
  {
    "pid": 322,
    "name": "sshd",
    "status": "Sleeping"
  },
  ...
]
```

---

## 🔧 DevOps Context

`procwatch` can be embedded in pipelines:

```yaml
- name: Check for running app
  run: ./target/release/procwatch --filter myservice
```

---

## 📜 License

MIT — Free to use and extend.

> Built by [Mike Niner](https://github.com/mikeninerbravog) with 🦀

