#!/usr/bin/env bash
set -e

cd "$(dirname "$0")"

cargo build --release -p purse -p purse-niri

cp target/release/purse ~/.local/bin/purse
cp target/release/purse-niri ~/.local/bin/purse-niri

echo "installed purse + purse-niri"
