#! bin/bash

trunk build yard/index.html
cargo run --package=yard --features=ssr --bin server -- --dir yard/dist
