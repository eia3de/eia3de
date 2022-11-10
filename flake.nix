{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.05";

    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit self; } {
      systems = [
        "x86_64-linux"
      ];

      perSystem = { lib, pkgs, inputs', ... }:
        let
          inherit (pkgs) pkgsBuildHost;
        in
        {
          devShells.default = pkgs.mkShell rec  {
            nativeBuildInputs = with pkgsBuildHost; [
              (inputs'.fenix.packages.complete.withComponents [
                "cargo"
                "clippy"
                "rust-analyzer"
                "rust-src"
                "rustc"
                "rustfmt"
              ])

              cargo-watch

              pkg-config
            ];

            buildInputs = with pkgs; [
              alsaLib
              libxkbcommon
              udev
              vulkan-loader
              wayland
              xlibsWrapper
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

            CARGO_TARGET_LINKER = "${pkgsBuildHost.clang}/bin/clang";
            CARGO_TARGET_RUSTFLAGS = "-C link-arg=-fuse-ld=${lib.getExe pkgsBuildHost.mold} -Z share-generics=y";
          };
        };
    };
}
