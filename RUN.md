#  Install and Run riichi-mahjong-calculator
#### **Note:** Initial `cargo run --release` builds may take up to 1~4 minutes to compile. Wait ~5 seconds for app to launch.
## Windows (MSYS2)

### 1. **Install Prerequisites**
- [Rust](https://rust-lang.org/tools/install/) including `rustup` and `cargo`
- [Git for Windows](https://git-scm.com)
- [MSYS2](https://www.msys2.org/) (UCRT64 environment). In MSYS2 UCRT64 terminal run:
```bash
    pacman -S --needed base-devel mingw-w64-ucrt-x86_64-toolchain
```
  **Note**: add the path (usually C:\msys64\usr\bin) to PATH
### 2. **Clone or Download**
```bash
    git clone https://github.com/Renyu-Liu/riichi-mahjong-calculator.git
    cd riichi-mahjong-calculator
```
_Or download the ZIP and `cd` into the extracted folder._
### 3. **Run**
```bash
    cargo run --release
```

## macOS

### 1. **Install Prerequisites**
- [Rust](https://rust-lang.org/tools/install/) (`rustup`, `cargo`)
- [Git](https://git-scm.com)
- Xcode Command Line Tools: `xcode-select --install`
### 2. **Clone or Download** (same as above)
### 3. **Run**
```bash
    cargo run --release
```

## Linux

### 1. **Install Prerequisites**
- [rustup + cargo](https://rust-lang.org/tools/install/)
- [Git](https://git-scm.com)
### 2. **Clone or Download** (same as above)
### 3. **Run**
```bash
    cargo run --release
```
