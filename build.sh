#!/bin/sh

#cargo build --release --target asmjs-unknown-emscripten
cargo rustc --release --target asmjs-unknown-emscripten -- -C opt-level=s
cp before.js demangle.js
cat target/asmjs-unknown-emscripten/release/cpp_demangle_js.js >> demangle.js
cat after.js >> demangle.js
echo
echo "done."
