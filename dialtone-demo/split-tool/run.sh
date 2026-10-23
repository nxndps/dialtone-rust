#!/bin/sh
# Trunk inherits NO_COLOR=true. cargo run is happier with the variable unset.
unset NO_COLOR
unset CARGO_TERM_COLOR
cd "$(dirname "$0")/../.." || exit 1
exec cargo run -p dialtone-demo-split --offline --target-dir /tmp/dt-demo-split-host --quiet
