{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { nixpkgs, ... }:
    let
      supportedSystems = [
        "x86_64-linux"
      ];

      overlays = [
        inputs.fenix.overlay
      ];

      forEachSystem = nixpkgs.lib.genAttrs supportedSystems;
      withPkgsForEachSystem = f: forEachSystem (system: f (import nixpkgs { inherit system overlays; }));
    in
    {
      devShell = withPkgsForEachSystem (pkgs: pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          pkgconfig
          clang
          lld
        ];

        buildInputs = with pkgs; [
          udev
          alsaLib
          vulkan-loader
          x11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          libxkbcommon
          wayland

          cargo-watch

          fenix.default.toolchain
        ];

        shellHook =
          let
            prepend = with pkgs; lib.makeLibraryPath [
              udev
              alsaLib
              vulkan-loader
              libxkbcommon
              wayland
            ];
          in
          ''
            export LD_LIBRARY_PATH="${prepend}''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
          '';

        # TODO: system -> triple mapping
        CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "clang";
        CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS = "-C link-arg=-fuse-ld=lld -Zshare-generics=y";
      });
    };
}
