#!/bin/bash
echo "Building Rust app..."
PATH=$PATH:/vercel/.cargo/bin

trunk build --release
