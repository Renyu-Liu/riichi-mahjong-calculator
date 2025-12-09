#  Build instructions
    > **Warning:** Initial `cargo run --release` builds may take up to ~4 minutes to compile.

## Windows (MSYS2)

1. **Install Prerequisites**
    - [Rust](https://rust-lang.org/tools/install/) including `rustup` and `cargo`
    - [Git for Windows](https://git-scm.com)
    - [MSYS2](https://www.msys2.org/) (UCRT64 environment). In MSYS2 UCRT64 terminal run:
      ```bash
      pacman -S --needed base-devel mingw-w64-ucrt-x86_64-toolchain
      ```
      Note: you must add the path (usually C:\msys64\usr\bin) to PATH
2. **Clone or Download**
    ```bash
    git clone https://github.com/Renyu-Liu/Riichi_Mahjong_Scoring_Calculator.git
    cd Riichi_Mahjong_Scoring_Calculator
    ```
    _Or download the ZIP and `cd` into the extracted folder._
3. **Run**
    ```bash
    cargo run --release
    ```

## macOS

1. **Install Prerequisites**
    - [Rust](https://rust-lang.org/tools/install/) (`rustup`, `cargo`)
    - [Git](https://git-scm.com)
    - Xcode Command Line Tools: `xcode-select --install`
2. **Clone or Download** (same as above)
3. **Run**
    ```bash
    cargo run --release
    ```

## Linux

1. **Install Prerequisites**
    - [rustup + cargo](https://rust-lang.org/tools/install/)
    - [Git](https://git-scm.com)
2. **Clone or Download** (same as above)
3. **Run**
    ```bash
    cargo run --release
    ```

