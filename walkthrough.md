# Refactoring yaku_checker.rs

I have refactored `src/implements/yaku_checker.rs` by splitting it into several smaller files within a new module `src/implements/yaku_checkers/`. This improves code organization and maintainability.

## Changes

### New Directory Structure

Created `src/implements/yaku_checkers/` with the following files:

- **`mod.rs`**: The main entry point for the module. It defines the `YakuResult` struct and the `check_all_yaku` function. It re-exports the other sub-modules.
- **`utils.rs`**: Contains helper functions used across different checkers, such as `count_dora`, `get_all_tiles`, `count_koutsu_kantsu`, etc.
- **`standard.rs`**: Contains logic for checking standard yaku (e.g., Pinfu, Tanyao, Yakuhai, Chinitsu).
- **`yakuman.rs`**: Contains logic for checking Yakuman (e.g., Kokushi Musou, Suuankou, Daisangen).

### Modified Files

- **`src/implements/mod.rs`**: Updated to use `yaku_checkers` instead of `yaku_checker`.
- **`src/implements/score_calculator.rs`**: Updated the import of `YakuResult` to reflect the new module structure.

### Deleted File

- **`src/implements/yaku_checker.rs`**: Removed as its contents have been distributed to the new files.

## Verification

Ran `cargo check` to ensure that the project compiles successfully with the new structure.
