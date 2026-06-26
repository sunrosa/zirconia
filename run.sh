nix-shell -p pkg-config libX11 libXi libXtst --run "RUST_BACKTRACE=1 cargo r"
