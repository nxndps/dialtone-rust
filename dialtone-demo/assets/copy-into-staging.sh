#!/bin/sh
# Trunk copies dialtone/assets during the build, in parallel with other assets.
# This runs in post_build, after that copy, and replaces the demo's logo URL.
set -eu
if [ -z "${TRUNK_STAGING_DIR:-}" ]; then
  echo "TRUNK_STAGING_DIR is unset; run this from the dialtone-demo Trunk post_build hook" >&2
  exit 1
fi
here=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
mkdir -p "$TRUNK_STAGING_DIR/dialtone/illustrations" "$TRUNK_STAGING_DIR/assets/images"
cp "$here/dialpad-logo.svg" "$TRUNK_STAGING_DIR/dialtone/illustrations/dialpad-logo.svg"
cp "$here/person.png" "$TRUNK_STAGING_DIR/assets/images/person.png"
