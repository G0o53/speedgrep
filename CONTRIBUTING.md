# 🌱 Contributing to speedgrep

Welcome! Before you open a PR, **please read this carefully.**

## 🚫 SIMD / Assembly / GPU PRs
Please do NOT submit PRs that:
- add SIMD intrinsics
- add handwritten assembly
- require AVX, AVX2, AVX-512, NEON, or any other CPU extensions
- offload searching to the GPU
- require a 2GB lookup table “for performance”
- turn speedgrep into a 400‑crate dependency monster
- add cleaner error messages

speedgrep aims to stay **small, simple, and portable**.
<br>
also **please don't add ANY dependencies to speedgrep, its a standalone package**

If your PR requires a PhD in microarchitecture to review, it will be closed with love.

## ✔️ PRs we DO accept
- bug fixes
- portability improvements
- correctness improvements
- small, elegant optimizations
- documentation improvements
- benchmarks (within reason)
- code clarity improvements
- unsafety for 0.2% performance boost

## 🧘 Philosophy
speedgrep is about:
- simplicity
- readability
- maintainability
- “fast enough without becoming a spaceship” **<- important**

If your PR makes speedgrep faster *and* smaller *and* cleaner, you are a hero.

If it makes speedgrep faster but turns it into a 20,000‑line SIMD labyrinth,  
please fork the project and name it **quickgrep**.

*Sincerely, G0o53*
