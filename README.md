# speedgrep

A no‑nonsense, literal‑string grep clone built in Rust.
![benchmark](docs/benchmark.gif)

## ⬇️  Download
```bash
cargo install speedgrep
```
this will install the `speedgrep` binary and the `quietgrep` binary

## 📘 Use
For `quietgrep`
```bash
quietgrep <file> <string>
```
that will return a 0 if the pattern if found, a 1 if not
<br>
<br>
For `speedgrep`
```bash
speedgrep <file> <string>
```
that will print all the line numbers of all the matches found

### 📝 To-Do
- [x] Increase speed and efficiency
- [ ] Add nicer colours without sacrificing performance
- [x] Make a cleaner output

## ✨ Features

- 🧠 Simple syntax — just `speedgrep <file> <string>`
- ⚡ Ridiculously fast
- 🔢 Shows line numbers
- 📦 Tiny binary 

# 🤝 Contributing
If YOU! Want to help, PR are welcome!
\
Please first read the [contribution guidelines for speedgrep](CONTRIBUTING.md)
\
Jump in here:

[![Repo](https://img.shields.io/badge/GitHub-speedgrep-blue)](https://github.com/G0o53/speedgrep)
\
[![Repo](https://img.shields.io/badge/Cargo-speedgrep-orange)](https://crates.io/crates/speedgrep)

## 🚀 We Beat Ripgrep

On a 1,000,000‑line test file:

- **⚡ speedgrep:** 361ms  
- **🦀 ripgrep:** 7.4 seconds

#### Note: Speedgrep is a speed demon, while ripgrep is a regex engine.

Minimalism wins.
