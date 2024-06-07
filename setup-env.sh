#!/bin/bash

# Clean everything: rm -rf ../../.local/share/solana && rm -rf ../../.cache/solana

RUSTVER="1.78"
SOLANAVER="1.18.5"
ANCHORVER="0.29.0"

# Quick change of solana version:
#solana-install init $SOLANAVER

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install $RUSTVER
rustup default $RUSTVER

curl -sSfL https://release.solana.com/v${SOLANAVER}/install | sh

cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install $ANCHORVER
avm use $ANCHORVER

