# speedgrep

A no‑nonsense, literal‑string grep clone built in Rust.
![benchmark](notcode/benchmark.gif)

## ⬇️  Download
```bash
cargo install speedgrep
```

### 📝 To-Do
- [x] Increase speed and efficiency
- [ ] Add nicer colours without sacrificing performance
- [x] Make a cleaner output

## ✨ Features

- 🧠 Simple syntax — just `speedgrep <file> <string>`
- ⚡ Ridiculously fast
- 🔢 Shows line numbers
- 📦 Tiny binary 

# 🧪 The Trick?

Why is it 20x faster? Because it avoids the "Regex Tax" and respects the hardware:

- 🚫 **NO REGEX:** Pure literal substring matching using Rust's optimized SIMD-backed `contains()`.
- 📦 **Syscall Batching:** Buffers 64 matches into a single `print!` to minimize expensive context switching to the Kernel.
- ♻️ **Buffer Reuse:** Reuses a single `String` buffer for the entire lifecycle, keeping the memory allocator silent.
- ⚡ **Single-Threaded Purity:** No thread-sync overhead; just raw, linear throughput.


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
