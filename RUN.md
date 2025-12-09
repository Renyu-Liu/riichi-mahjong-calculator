#  Install and Run riichi-mahjong-calculator
**Note:** Initial `cargo run --release` builds may take up to 1~4 minutes to compile. Wait ~5 seconds for app to launch.


## 1. **Install Prerequisites**
### Windows
- [Rust](https://rust-lang.org/tools/install/) including `rustup` and `cargo`
- [Git for Windows](https://git-scm.com)
- [Visual Studio](https://visualstudio.microsoft.com) (C++ Desktop Development) OR [MSYS2](https://www.msys2.org/) (UCRT64 environment).

In MSYS2 UCRT64 terminal run:
```bash
    pacman -S --needed base-devel mingw-w64-ucrt-x86_64-toolchain
```
  **Note**: add the path (usually C:\msys64\usr\bin) to PATH
  
### macOS
- [Rust](https://rust-lang.org/tools/install/) (`rustup`, `cargo`)
- [Git](https://git-scm.com)
- Xcode Command Line Tools: `xcode-select --install`

### Linux
- [rustup + cargo](https://rust-lang.org/tools/install/)
- [Git](https://git-scm.com)

### 2. **Git Clone or Download**
**Note**: please include `--depth 1` to save your downloading time.
```bash
    git clone --depth 1 https://github.com/Renyu-Liu/riichi-mahjong-calculator.git
    cd riichi-mahjong-calculator
```
_Or download the ZIP and `cd` into the extracted folder._

### 3. Run
```bash
    cargo run --release
```
