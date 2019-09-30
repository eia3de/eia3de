{ pkgs ? import <nixpkgs> {}
}:

with pkgs;

mkShell {
  buildInputs = [
    alsaLib
    cmake
    freetype
    latest.rustChannels.stable.rust
    expat
    openssl
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

  RUSTFLAGS = "-C target-cpu=native";

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
    export PATH="$PATH:$CARGO_HOME/bin"
  '';
}
