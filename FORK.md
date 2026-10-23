# MonoTerm maintenance fork

This fork of [Xuanwo/gpui-ghostty](https://github.com/Xuanwo/gpui-ghostty) is maintained at [cod7ce/gpui-ghostty](https://github.com/cod7ce/gpui-ghostty) for MonoTerm. The maintenance branch is `monoterm`, based on upstream commit `e3025981c6211dd7db2a825dc364ffb5d342f45e`.

## Downstream changes

- Add installed Nerd Font families to the terminal fallback list while retaining the platform's primary monospace font. Fonts are not bundled.
- Forward erase-character, insert-line, delete-line and scrolling-region operations from the VT stream handler to Ghostty. These fix stale text and repeated rows in Neo-tree.
- Cover those VT operations in `crates/gpui_ghostty_terminal/tests/terminal_edits.rs`.

The Ghostty submodule and upstream GPUI lockfile remain at their existing revisions. MonoTerm uses a local GPUI path substitution to share its GPUI instance with this crate; that environment-specific substitution is deliberately not part of this fork.

## Maintenance

Keep reusable terminal fixes and their tests on `monoterm`. Preserve the upstream Git history and licenses. Fetch upstream changes for review, then merge or cherry-pick deliberately and rerun the workspace checks. Do not force-push the maintenance branch.

Consumers should pin a full commit SHA, not the moving branch name. Updating this branch does not automatically upgrade MonoTerm: change MonoTerm's `scripts/bootstrap.sh` revision only after testing the selected commit. Keep its application integration tests as a compatibility check in addition to this fork's tests.

The initial fixes were verified in MonoTerm on Apple Silicon macOS with the installed LazyVim configuration, first-render screenshots, window/font resizing, and the terminal edit regression tests. Workspace checks follow the upstream `AGENTS.md` instructions; macOS also needs a compatible Zig SDK and GPUI's Metal toolchain or `runtime_shaders` feature.

On the initial macOS 27 development host, all 50 tests in `ghostty_vt` and `gpui_ghostty_terminal` passed using MonoTerm's dependency graph. Workspace Rustfmt and Clippy for all three crates also passed; the direct workspace Clippy run required replacing fixed-size `chunks_exact` loops with `as_chunks`. The additional `ghostty_vt_sys` unit-test executable (which contains no Rust tests) failed to link because the system linker rejected Zig's static archive member alignment. The same FFI archive linked into and ran the VT and terminal tests above. Standalone fork CI results must be checked separately; these local results do not establish a clean CI run. The macOS job uses `macos-15` with Xcode 16.4 so Zig 0.14.1 does not consume newer incompatible SDK stubs. CI pins Rust 1.98.1, matching the development toolchain, and uses `--locked` to retain the GPUI baseline. Toolchain updates are reviewed explicitly so a new Clippy release does not unexpectedly change the maintenance branch's check policy.
