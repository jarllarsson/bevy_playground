My Bevy Playground
---
Playing around with Bevy and Rust, learning, failing... borrow checking...?

Each file is probably going to be its own contained little example.

Run a file with

``cargo run --example name``

*Or cargo build to just build it.*

To debug in VSCode, open the folder as a project in VSCode, and press F5 on the .rs file to debug. The current opened file will be built and debugged (see launch.json).
Debug is setup in this fashion for both MSVC and LLDB.

VSCode plugins:
* "rust-analyzer" | Rust
* "CodeLLDB" and/or "C/C++" | Debugger


Uses custom build.rs script from https://stackoverflow.com/questions/57535794/how-do-i-include-a-folder-in-the-building-process to get any pesky assets in the right place on build.

:crab: 	:dove: