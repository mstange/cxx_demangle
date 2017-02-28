#!/bin/sh

cargo build --release --target asmjs-unknown-emscripten
cp before.js demangle.js
cat target/asmjs-unknown-emscripten/release/cpp_demangle_js.js >> demangle.js
cat after.js >> demangle.js
echo
echo "done."
