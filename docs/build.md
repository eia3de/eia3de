# Build Instructions


## Dependencies

- [Stable Rust](https://www.rust-lang.org/tools/install)


### NixOS/nixpkgs

```nix
{ pkgs ? import <nixpkgs> {}
}:

with pkgs;

mkShell {
  buildInputs = [
    latest.rustChannels.stable.rust

    pkgconfig
    xlibs.libX11
    cmake
  ];

  APPEND_LD_LIBRARY_PATH = lib.concatMapStringsSep ":" (x: x + "/lib") (with pkgs; [
    vulkan-loader
    libGL
    xlibs.libX11
    xlibs.libXcursor
    xlibs.libXi
    xlibs.libXrandr
    xlibs.libXxf86vm
    xorg.libxcb
  ]);

  shellHook = ''
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$APPEND_LD_LIBRARY_PATH
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
