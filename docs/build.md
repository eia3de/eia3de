# Build Instructions


## Dependencies

- [Stable Rust](https://www.rust-lang.org/tools/install)


### NixOS/nixpkgs

- [Rust overlay](https://github.com/mozilla/nixpkgs-mozilla#rust-overlay)

```nix
{ pkgs ? import <nixpkgs> {}
}:

with pkgs;

mkShell {
  buildInputs = [
    cmake
    latest.rustChannels.stable.rust
    pkgconfig
    python3
    vulkan-validation-layers
    xlibs.libX11
  ];

  APPEND_LIBRARY_PATH = pkgs.stdenv.lib.makeLibraryPath [
    vulkan-loader
    xlibs.libXcursor
    xlibs.libXi
    xlibs.libXrandr
  ];

  # RUSTC_WRAPPER = "sccache";
  RUSTFLAGS = "-C target-cpu=native";

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
    export PATH="$PATH:$HOME/.cargo/bin"
  '';
}
```


### Optional but suggested dependencies

- [`sccache`](https://github.com/mozilla/sccache#installation)


## Building and Running

```
git clone https://github.com/eia3de/eia3de
cd eia3de
cargo build --release
cargo run --release --bin eia3de_client
```
